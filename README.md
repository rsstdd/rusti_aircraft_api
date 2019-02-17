# Aircraft Api Written in Rust

A RESTFUL Aircraft API.

## Getting Started
In order to run this application, you'll need to clone the repo, and execute `cargo run`.


### Prerequisites
* Rust Lang
	- The preferred way to install Rust is via `rustup`. Instructions for how to do so may be found [here](https://www.rust-lang.org/tools/install).

### Installing

Clone the repo

```
git clone git@github.com:rsstdd/rusti_aircraft_api.git
cd rusti_aircraft_api
cargo run
```

## Usage
__Using [HTTPie](https://httpie.org/)__

Aircraft

GET `/aircraft/`
```
http localhost:8000/api/aircraft/

[
  {
    "ceiling": 12000,
    "country_of_origin": "United States",
    "description": "The Piper J-3 Cub is a single-engine two-seat light touring aircraft and military trainer and liaison aircraft produced by the US-American manufacturer Piper Aircraft Corporation. The US Army variants were initially designated O-59 and later L-4 Grasshopper, US Navy designation of the J-3 was NE-1. The Piper J-3 Cub is a development of the Taylor J-2 Cub. 5703 of the total 19828 built J-3 were military aircraft.",
    "engines": "1 piston 65hp Contenental A-65",
    "id": 1,
    "img_url": "https://airandspace.si.edu/sites/default/files/styles/slideshow_sm/public/images/collection-objects/record-images/A19771128000cp10.JPG?itok=26px07OX",
    "max_range": 250,
    "max_speed": 92,
    "name": "Piper J-3",
    "operators": "United States, Brazil",
    "year_in_service": 1938
  },
  {
    "ceiling": 21500,
    "country_of_origin": "United States",
    "description": "The North American T-6 Texan is a single-engine two-seat trainer aircraft produced by the US-American manufacturer North American Aviation. The T-6 was designated AT-6 by the United States Army Air Corps (USAAC) and later by the United States Army Air Forces (USAAF), SNJ by the US Navy and Harvard by the British Commonwealth air forces. The AT-6 was redesignated T-6 in 1948 by the USAF. The T-6 Texan is the world's most operated trainer aircraft ever. The T-6 was also license-built in Canada by Noorduyn Aviation and Canada Car and Foundry.",
    "engines": "1 Radial 600hp Pratt & Whitney R-1340-AN-1 Wasp",
    "id": 3,
    "img_url": "https://upload.wikimedia.org/wikipedia/commons/6/64/Fap-instrucao-pilotos-north-american-t-6-texan.jpg",
    "max_range": 750,
    "max_speed": 205,
    "name": "North American T-6 Texan / AT-6 / SNJ / Harvard",
    "operators": "United States, Brazil, Canada, India, New Zealand, United Kingdom, Norway",
    "year_in_service": 1935
  }
		.
		.
		.
]
```

GET `/aircraft/<id>`
```
http localhost:8000/api/aircraft/22/

[
  {
    "ceiling": 42000,
    "country_of_origin": "United States",
    "description": "The Republic P-47 Thunderbolt is a single-engine single-seat fighter and fighter-bomber aircraft produced by the US-American manufacturer Republic Aviation Corporation.",
    "engines": "1 Radial 2500hp Pratt & Whitney R-2800-59W Double Wasp",
    "id": 22,
    "img_url": "https://upload.wikimedia.org/wikipedia/commons/c/c4/Republic_P-43_Lancer.jpg",
    "max_range": 475,
    "max_speed": 433,
    "name": "Republic P-47 Thunderbolt",
    "operators": "US, Australia, China",
    "year_in_service": 1941
  }
]

```

POST `/aircraft/<id>`

Input data saved as file.json
```
{
  "ceiling": 11200,
  "country_of_origin": "United States",
  "description": "The Stearman Model 75 Kaydet is a single-engine two-seat trainer biplane aircraft produced by the US-American manufacturer Stearman Aircraft Company and later by the Boeing Airplane Company. PT-13 / PT-17 / PT-18 were the main-variants used by the USAAC, NS-1 and NS2S for the US Navy. The Stearman 75 Kaydet was used by the US Army Air Corps and the US Navy as a primary/basic trainer aircraft. After WWII many Stearman 75 were used as crop-duster aircraft due to their slow and low-level flight capability.",
  "engines": "1 Radial 220 hp Lycoming R-680-17",
  "id": 2,
  "img_url": "http://www.flugzeuginfo.net/acimages/pt13dn_kp.jpg",
  "max_range": 505,
  "max_speed": 124,
  "name": "Boeing-Stearman Kaydet PT-13 / PT-17 / PT-18 / NS-1 / N2S (Model 75)",
  "operators": "United States, Brazil, Canada, China, Phillipines",
  "year_in_service": 1934
}

```

```
http post localhost:8000/api/aircraft/2/ < file.json

{
  "ceiling": 11200,
  "country_of_origin": "United States",
  "description": "The Stearman Model 75 Kaydet is a single-engine two-seat trainer biplane aircraft produced by the US-American manufacturer Stearman Aircraft Company and later by the Boeing Airplane Company. PT-13 / PT-17 / PT-18 were the main-variants used by the USAAC, NS-1 and NS2S for the US Navy. The Stearman 75 Kaydet was used by the US Army Air Corps and the US Navy as a primary/basic trainer aircraft. After WWII many Stearman 75 were used as crop-duster aircraft due to their slow and low-level flight capability.",
  "engines": "1 Radial 220 hp Lycoming R-680-17",
  "id": 33,
  "img_url": "http://www.flugzeuginfo.net/acimages/pt13dn_kp.jpg",
  "max_range": 505,
  "max_speed": 124,
  "name": "Boeing-Stearman Kaydet PT-13 / PT-17 / PT-18 / NS-1 / N2S (Model 75)",
  "operators": "United States, Brazil, Canada, China, Phillipines",
  "year_in_service": 1934
}
```

UPDATE `/aircraft/<id>`

Input data saved as file.json
```
{
  "ceiling": 11200,
  "country_of_origin": "United States",
  "description": "The Stearman Model 75 Kaydet is a single-engine two-seat trainer biplane aircraft produced by the US-American manufacturer Stearman Aircraft Company and later by the Boeing Airplane Company. PT-13 / PT-17 / PT-18 were the main-variants used by the USAAC, NS-1 and NS2S for the US Navy. The Stearman 75 Kaydet was used by the US Army Air Corps and the US Navy as a primary/basic trainer aircraft. After WWII many Stearman 75 were used as crop-duster aircraft due to their slow and low-level flight capability.",
  "engines": "1 Radial 220 hp Lycoming R-680-17",
  "id": 2,
  "img_url": "http://www.flugzeuginfo.net/acimages/pt13dn_kp.jpg",
  "max_range": 505,
  "max_speed": 124,
  "name": "Boeing-Stearman Kaydet PT-13 / PT-17 / PT-18 / NS-1 / N2S (Model 75)",
  "operators": "United States, Brazil, Canada, China, Phillipines",
  "year_in_service": 1934
}

```

```
http put localhost:8000/api/aircraft/2/ < file.json

{
  "ceiling": 11200,
  "country_of_origin": "United States",
  "description": "The Stearman Model 75 Kaydet is a single-engine two-seat trainer biplane aircraft produced by the US-American manufacturer Stearman Aircraft Company and later by the Boeing Airplane Company. PT-13 / PT-17 / PT-18 were the main-variants used by the USAAC, NS-1 and NS2S for the US Navy. The Stearman 75 Kaydet was used by the US Army Air Corps and the US Navy as a primary/basic trainer aircraft. After WWII many Stearman 75 were used as crop-duster aircraft due to their slow and low-level flight capability.",
  "engines": "1 Radial 220 hp Lycoming R-680-17",
  "id": 33,
  "img_url": "http://www.flugzeuginfo.net/acimages/pt13dn_kp.jpg",
  "max_range": 505,
  "max_speed": 124,
  "name": "Boeing-Stearman Kaydet PT-13 / PT-17 / PT-18 / NS-1 / N2S (Model 75)",
  "operators": "United States, Brazil, Canada, China, Phillipines",
  "year_in_service": 1934
}
```


DELETE `/aircraft/<id>`

```
http delete localhost:8000/api/aircraft/2/

true
```



## Built With

* [Rocket](https://rocket.rs/) - Rust Web Framework
* [Diesel](http://diesel.rs/) - ORM
* [PSQL](https://www.postgresql.org/) - DB

## Authors

* **Ross Todd** - *Initial work* - [rsstdd](https://github.com/rsstdd)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
