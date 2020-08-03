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
	nb_place   int   not null,
	created_at timestamp not null default current_timestamp(),
	updated_at timestamp not null default current_timestamp(),
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
INSERT INTO city
(name, postal_code, country_id)
VALUES ('Lille', '59000', 5);

INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.631778, 3.045893, 1, 1);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.621754, 3.036441, 1, 2);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.619691, 3.039200, 1, 1);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.619651, 3.036204, 1, 5);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.629694, 3.039409, 1, 2);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.628691, 3.039200, 2, 9);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.619612, 3.049300, 1, 1);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.629291, 3.039203, 3, 1);
INSERT INTO place (longitude, latitude, city_id, nb_place)
VALUES (50.619202, 3.039900, 1, 1);
INSERT INTO place (longitude,latitude, city_id, nb_place)
VALUES (50.619291, 3.039700, 5, 2);
