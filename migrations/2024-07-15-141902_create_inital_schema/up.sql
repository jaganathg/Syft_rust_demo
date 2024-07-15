CREATE TABLE component (
    id INTEGER PRIMARY KEY,
    syft_name TEXT NOT NULL,
    black_duck_name TEXT NOT NULL,
    type TEXT NOT NULL,
    version TEXT NOT NULL,
    download_link TEXT NOT NULL,
    distribution TEXT NOT NULL,
    use_linkage TEXT NOT NULL,
    use_modification TEXT NOT NULL,
    license_text TEXT NOT NULL,
    purl TEXT NOT NULL,
    cpe TEXT NOT NULL,
    entry_date TIMESTAMP NOT NULL,
    last_update TIMESTAMP NOT NULL
);

CREATE TABLE component_release (
    id INTEGER PRIMARY KEY,
    component_id INTEGER NOT NULL,
    release_id INTEGER NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 0,
    notes TEXT NOT NULL CHECK (LENGTH(notes) <= 100),
    entry_date TIMESTAMP NOT NULL,
    last_update TIMESTAMP NOT NULL,
    FOREIGN KEY (component_id) REFERENCES component(id)
);

CREATE TABLE release (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL CHECK (LENGTH(name) <= 10),
    release_date DATE NOT NULL,
    cluster INTEGER NOT NULL,
    project TEXT NOT NULL,
    contact TEXT
);

CREATE TABLE component_license (
    id INTEGER PRIMARY KEY,
    component_id INTEGER NOT NULL,
    license_id INTEGER NOT NULL,
    FOREIGN KEY (component_id) REFERENCES component(id),
    FOREIGN KEY (license_id) REFERENCES license(id)
);

CREATE TABLE license (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    spdx TEXT NOT NULL,
    standard_text TEXT NOT NULL
);

CREATE TABLE component_author (
    id INTEGER PRIMARY KEY,
    component_id INTEGER NOT NULL,
    author_id INTEGER NOT NULL,
    FOREIGN KEY (component_id) REFERENCES component(id),
    FOREIGN KEY (author_id) REFERENCES author(id)
);

CREATE TABLE author (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);