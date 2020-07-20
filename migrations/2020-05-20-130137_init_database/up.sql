-- Your SQL goes here
create table country
(
    id   int auto_increment
        primary key,
    name varchar(50) not null
);



create table city
(
    id          int auto_increment
        primary key,
    name        varchar(50) not null,
    postal_code int         not null,
    country_id  int(6)      not null,
    constraint city_country_id_fk
        foreign key (country_id) references country (id)
);

create table place
(
    id         int auto_increment
        primary key,
    longitude  float not null,
    latitude   float not null,
    city_id    int   not null,
    created_at timestamp default current_timestamp(),
    updated_at timestamp default current_timestamp(),
    constraint place_city_id_fk
        foreign key (city_id) references city (id)
);

INSERT INTO country (name)
VALUES ('FRANCE');
INSERT INTO country (name)
VALUES ('ENGLAND');
INSERT INTO country (name)
VALUES ('GERMANY');
INSERT INTO country (name)
VALUES ('SPAIN');
INSERT INTO country (name)
VALUES ('BELGIUM');
INSERT INTO country (name)
VALUES ('USA');

INSERT INTO city
    (name, postal_code, country_id)
VALUES ('Paris', '75000', 1);
INSERT INTO city
    (name, postal_code, country_id)
VALUES ('Rueil-Malmaison', '92500', 1);
INSERT INTO city
    (name, postal_code, country_id)
VALUES ('Bruxelles', '99', 5);
INSERT INTO city
    (name, postal_code, country_id)
VALUES ('Barcelona', '11', 4);

INSERT INTO place (longitude, latitude, city_id)
VALUES (50.631778, 3.045893, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.621754, 3.036441, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
INSERT INTO place (longitude, latitude, city_id)
VALUES (50.619691, 3.039200, 1);
