# pokemon-shakespeare

pokemon-shakespeare is a simple REST API written in Rust that given a pokemon name returns its description translated to Shakespearean style. 
The official pokemon description is obtained from [Pokeapi](https://pokeapi.co/) using the [Pokerust](https://gitlab.com/lunik1/pokerust) wrapper. Once the description
is obtained the [funtranslations](https://funtranslations.com/api/shakespeare) API is called in order to get the translated version of the pokemon description.

## Getting Started

The REST API was build using [Rocket](https://rocket.rs/v0.4/). The following steps will get you the necessary set-up to run the application:
* Install [Rust](https://www.rust-lang.org/tools/install)
* Once Rust is installed, select the nighly version as default: `rustup default nightly`
* Clone the [pokemon-shakespeare](https://github.com/felipe07/pokemon-shakespeare/) repository
* Run `cargo run` to install dependencies and run the application
* Tests can be executed with: cargo test

### Prerequisites

Apart from Rust itself no particular prerequisites are necessary if you run the application on Mac OS or GNU/Linux. If you use Windows you will need to instal the .Net build tools in order to compile and run Rust binaries.

## Getting the pokemon description in shakespeare

`curl --location --request GET 'http://localhost:8000/pokemon/pikachu'`

The response should be:

```javascript
{
  "name": "pikachu",
  "description": "Whenever pikachu cometh across something new, 't blasts 't with a jolt of electricity. If 't be true thee cometh across ablackened berry,  't’s evidence yond this pokémon did misprision theintensity of its charge."
}
```

## Running the tests

`cargo test`

## Built With

* [Rocket](https://rocket.rs/v0.4/) - The web framework used
* [Pokerust](https://gitlab.com/lunik1/pokerust) - Wrapper for [Pokeapi](https://pokeapi.co/)

## Things to improve

* Given that [funtranslations](https://funtranslations.com/api/shakespeare) is a paid service that allows a maximum of 5 requests per hour it would be better to cache the translated pokemon descriptions obtained from their website, that way most of the pokemon descriptions will be eventually obtained from local cache.

