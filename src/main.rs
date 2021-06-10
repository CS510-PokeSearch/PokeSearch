//! Jonathan Rivera and Tram Vuong 2021
//!
//! Sources:
//! https://medium.com/@james_32022/rocket-frontend-templates-and-static-assets-5b6d04243a08
//! https://rocket.rs/v0.4/guide/
//! https://www.youtube.com/watch?v=2RWXeosWhAQ
//! https://www.shawntabrizi.com/code/combining-rocket-with-reqwest-to-call-an-api-with-rust/
//! https://github.com/sunng87/handlebars-rust

#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate serde;

#[macro_use]
extern crate rocket;
extern crate serde_derive;
extern crate serde_json;

use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use rocket::request::LenientForm;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{handlebars, Template};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

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
        format!(
            "{}{}",
            (&string[..1].to_string()).to_uppercase(),
            &string[1..]
        )
    }

    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Invalid param."))?;

    let param_cap = cap_first(param.value().as_str().unwrap());

    out.write(param_cap.as_ref())?;
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
        .ok_or_else(|| RenderError::new("Invalid param."))?;

    let new_param = param.value().as_i64().unwrap() + 1;

    let rendered = format!("{}", new_param);
    out.write(rendered.as_ref())?;
    Ok(())
}

// struct to grab search query
#[derive(FromForm)]
struct SearchForm {
    pokemon: String,
}

// struct for invalid search
#[derive(Serialize, Debug)]
struct InvalidPokemon {
    query: String,
}

// index page routing
#[get("/")]
fn index() -> Template {
    let data: HashMap<&str, &str> = [("name", "PokeSearch")].iter().cloned().collect();

    Template::render("index", &data)
}

// post for search form
#[post("/search", data = "<form>")]
fn search_form(form: LenientForm<SearchForm>) -> Redirect {
    let mut pokemon = form.pokemon.clone();
    pokemon.retain(|c| !c.is_whitespace());
    Redirect::to(format!("/search/{}", pokemon))
}

// search page routing
// gets response from poke api
#[get("/search/<pokemon>")]
fn search(pokemon: String) -> Template {
    let base_url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    let full_url = &base_url[..];
    let client = reqwest::blocking::Client::new();

    let response = client.get(full_url).send().unwrap();

    if response.status().is_success() {
        let data: JsonValue = response.json().unwrap();
        Template::render("search", &data)
    } else {
        let data = InvalidPokemon { query: pokemon };
        Template::render("invalidsearch", &data)
    }
}

// firstgen page routing
#[get("/firstgen")]
fn firstgen() -> Template {
    let url = "https://pokeapi.co/api/v2/pokemon/?limit=151";
    let client = reqwest::blocking::Client::new();

    let response = client.get(url).send().unwrap();

    let data: JsonValue = response.json().unwrap();

    Template::render("firstgen", &data)
}

// catch 404 and render our own 404 page template
#[catch(404)]
fn not_found() -> Template {
    let data: HashMap<&str, &str> = [("text", "Looks like you got a little lost.")]
        .iter()
        .cloned()
        .collect();

    Template::render("not_found", &data)
}

// index page routing
#[get("/about")]
fn about() -> Template {
    let data = "";
    Template::render("about", &data)
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/img", StaticFiles::from("img"))
        .mount("/", routes![index, search, search_form, firstgen, about])
        .register(catchers![not_found])
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("capitalize", Box::new(capitalize));

            engines
                .handlebars
                .register_helper("add_one", Box::new(add_one));
        }))
        .launch();
}
