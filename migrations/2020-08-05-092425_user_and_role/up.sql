-- Your SQL goes here
create table role (
  id int auto_increment primary key,
  name varchar(50) not null
);
INSERT INTO
  role (name)
VALUES
  ('ADMIN');
INSERT INTO
  role (name)
VALUES
  ('EDITOR');
INSERT INTO
  role (name)
VALUES
  ('READER');
create table user (
  id int auto_increment,
  name varchar(50) not null,
  password varchar(255) not null,
  token varchar(255),
  role_id int not null,
  created_at timestamp not null default current_timestamp(),
  updated_at timestamp not null default current_timestamp(),
  constraint pk_user primary key (id, name),
  constraint role_id_fk foreign key (role_id) references role (id)
);
INSERT INTO
  user (name, password, role_id)
VALUES
  ('rolfie', 'rolfie', 1);
INSERT INTO
  user (name, password, role_id)
VALUES
  ('test-editor', 'test-editor', 2);
INSERT INTO
  user (name, password, role_id)
VALUES
  ('test-reader', 'test-reader', 3);
