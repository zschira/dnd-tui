create table spells (
    id int not null primary key,
    name varchar(32) not null,
    description text not null,
    higher_level text,
    range text,
    verbal boolean not null,
    somatic boolean not null,
    material boolean not null,
    material_text text,
    ritual boolean not null,
    duration varchar(32) not null,
    concentration boolean not null,
    casting_time varchar(32) not null,
    level int not null,
    school varchar(32) not null,
    classes text not null,
    subclasses text not null
);
