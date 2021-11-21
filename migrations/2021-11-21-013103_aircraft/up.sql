DROP TABLE IF EXISTS countries;


DROP TABLE IF EXISTS manufacturers;


DROP TABLE IF EXISTS aircraft_manufacturers;


DROP TABLE IF EXISTS powerplants;


DROP TABLE IF EXISTS aircrafts;


DROP TABLE IF EXISTS aircrafts_countries;


CREATE TABLE countries (id SERIAL PRIMARY KEY NOT NULL,
                                              country_codes varchar(2) NOT NULL,
                                                                       country_name varchar(75) NOT NULL,
                                                                                                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                      updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);


CREATE TABLE manufacturers (id SERIAL PRIMARY KEY NOT NULL,
                                                  country_of_origin_id INT REFERENCES countries (countries_id),
                                                                                      manufacturer_name VARCHAR NOT NULL);


CREATE TABLE powerplants (id SERIAL PRIMARY KEY NOT NULL,
                                                manufacturer_id VARCHAR REFERENCES aircraft_manufacturers(id));


CREATE TABLE aircrafts_countries (aircrafts_id INT REFERENCES aircrafts(id),
                                                              countries_id INT REFERENCES countries(id),
                                                                                          PRIMARY KEY (aircrafts_id,
                                                                                                       countries_id));


CREATE TABLE aircrafts (id SERIAL PRIMARY KEY NOT NULL,
                                              name varchar(255) NOT NULL,
                                                                description VARCHAR, year_in_service INT NOT NULL,
                                                                                                         country_of_origin VARCHAR REFERENCES countries (countries_id),
                                                                                                                                              max_speed INT NOT NULL,
                                                                                                                                                            max_range INT NOT NULL,
                                                                                                                                                                          ceiling INT NOT NULL,
                                                                                                                                                                                      powerplant_id VARCHAR REFERENCES powerplants (powerplants_id),
                                                                                                                                                                                                                       powerplant_number integer, img_url VARCHAR, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                                                                                                                                         updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);


CREATE TABLE aircrafts_manufacturers (id SERIAL PRIMARY KEY NOT NULL,)
