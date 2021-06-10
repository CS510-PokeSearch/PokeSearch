# PokéSearch
### Created by: Jonathan Rivera and Tram Vuong

PokéSearch is a web app built with Rust, [Rocket web framework](https://rocket.rs/), [Reqwest](https://crates.io/crates/reqwest), and [Handlebars templating](https://crates.io/crates/handlebars). The app allows users to search for Pokemon and display information about them. This information is obtained by fetching data from [PokeApi](https://pokeapi.co/). 

The app is deployed using Render at https://rust-pokesearch.onrender.com/.

**The app requires the use of nightly Rust.** We tried to get everything working on stable Rust, but unfortunately we ran into roadblock after roadblock.

We did a large portion of the project by working together simultaneously in order to get the framework up and running and develop an understanding of Rocket and Handlebars. Testing was done primarily via local hosting. We also had beta testers dig around the site to locate any issues or oversights. This led to finding problems such as inputting spaces in a search query.

### Code Example - passing data to a Handlebars template after catching a 404 error using Rocket's routing:

    #[catch(404)]
    fn  not_found() -> Template {
    	let  data: HashMap<&str, &str> = [("text", "Looks like you got a little lost.")]
    		.iter().cloned().collect();
    
    	Template::render("not_found", &data)
    }

## Reflections
As expected, there were roadblocks along the way. As mentioned earlier, we had a lot of trouble trying to get the app running on stable Rust. We tried several workarounds, but nothing stuck. Finding the correct combination of versions across the different crates we used may have eventually gotten us on the right path, but we opted to avoid the headache and timesink. Also, we are both still beginners when it comes to using Git, so that came with its own set of hiccups.

Overall, we are very happy with how our project turned out. We'd love to revisit this project and hit some stretch goals. One major stretch goal we were unable to attain was a "compare pokémon" feature. We realized this was going to require another layer of fetches to the API and a restructuring of a decent chunk of our code. Another stretch goal that would be nice to have would be a dropdown where the user can select what generation they want to display. Currently, the app only allows for the user to display Gen I.
