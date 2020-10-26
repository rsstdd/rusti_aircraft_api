CREATE TABLE airplanes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    year_in_service VARCHAR NOT NULL,
    country_of_origin VARCHAR NOT NULL,
    operators VARCHAR NOT NULL,
    max_speed VARCHAR NOT NULL,
    max_range VARCHAR NOT NULL,
    ceiling VARCHAR NOT NULL,
    engines VARCHAR NOT NULL,
    img_url VARCHAR NOT NULL
);
