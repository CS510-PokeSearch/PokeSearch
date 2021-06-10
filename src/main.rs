//! Jonathan Rivera and Tram Vuong 2021
//! 
//! Sources:
//! https://medium.com/@james_32022/rocket-frontend-templates-and-static-assets-5b6d04243a08
//! https://rocket.rs/v0.4/guide/
//! https://www.youtube.com/watch?v=2RWXeosWhAQ
//! https://www.shawntabrizi.com/code/combining-rocket-with-reqwest-to-call-an-api-with-rust/
//! https://github.com/sunng87/handlebars-rust
//! https://pokeapi.co/

#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
extern crate reqwest;

#[macro_use] 
extern crate rocket;
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{Template, handlebars};
use handlebars::{Helper, Handlebars, Context, RenderContext, Output, RenderError};
use std::collections::HashMap;
use rocket::request::LenientForm;
use serde_json::Value as JsonValue;
use rocket::response::Redirect;
use rocket::Request;
use serde::Serialize;

// handlebars helper to capitalize first letter of strings
fn capitalize(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {

    // helper function to capitalize first letter of str
    fn cap_first(string: &str) -> String {
        format!("{}{}", (&string[..1].to_string()).to_uppercase(), &string[1..])
    }

    let param = h
        .param(0)
        .ok_or(RenderError::new("Invalid param."))?;

    let param_cap = cap_first(param.value().as_str().unwrap());
    
    let rendered = format!("{}", param_cap);
    out.write(rendered.as_ref())?;
    Ok(())
}

// handlebars helper to add 1 to index for displaying pokemon ID
fn add_one(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {

    let param = h
        .param(0)
        .ok_or(RenderError::new("Invalid param."))?;

    let new_param = param.value().as_i64().unwrap() + 1;
    
    let rendered = format!("{}", new_param);
    out.write(rendered.as_ref())?;
    Ok(())
}

// struct to grab search query from form
#[derive(FromForm)]
struct SearchForm{
    pokemon: String
}

// struct for invalid search
#[derive(Serialize, Debug)]
struct InvalidPokemon {
    query: String
}

// index page routing
#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("name", "PokeSearch")]
        .iter().cloned().collect();

    Template::render("index", &context)
}

// post for search form
#[post("/search", data = "<form>")]
fn search_form(form: LenientForm<SearchForm>) -> Redirect {
    Redirect::to(format!("/search/{}", form.pokemon))
}


// search page routing
// gets response from poke api
#[get("/search/<pokemon>")]
fn search(pokemon: String) -> Template {
    let base_url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    let full_url = &base_url[..];
    let client = reqwest::blocking::Client::new();

    let response = client.get(full_url)
        .send()
        .unwrap(); 

    if response.status().is_success() {
        let data: JsonValue = response.json().unwrap();
        Template::render("search", &data)
    } else { //if the response fails, render the invalidsearch page
        let data = InvalidPokemon {
            query: pokemon,
        };
        Template::render("invalidsearch", &data)
    }
}

// firstgen page routing
#[get("/firstgen")]
fn firstgen() -> Template {
    let base_url = format!("https://pokeapi.co/api/v2/pokemon/?limit=151");
    let full_url = &base_url[..];
    let client = reqwest::blocking::Client::new();

    let response = client.get(full_url)
        .send()
        .unwrap(); 

    let data: JsonValue = response.json().unwrap();

    Template::render("firstgen", &data)
}

// catch 404 and render our own 404 page template
#[catch(404)]
fn not_found() -> Template {
    let context: HashMap<&str, &str> = [("text", "Looks like you got a little lost.")]
        .iter().cloned().collect();

    Template::render("not_found", &context)
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/img", StaticFiles::from("img"))
        .mount("/", routes![index, search, search_form, firstgen])
        .register(catchers![not_found])
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("capitalize", Box::new(capitalize));
            engines.handlebars.register_helper("add_one", Box::new(add_one));
        }))
        .launch();
}