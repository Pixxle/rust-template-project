# Rust-template-project

A simple multi-threaded web server implemented in rust, it tries to follow the onion-architecture standard to some sense with repositories implementing traits to easily be mockable.
There are some things i think could be done a lot better still, such as how i do the configuration parsing. But I'm fairly happy with how the internal mutability with mutex works for the in_mem implementation of the database. Of course a real database service would not require any mutex at all, but this is nice because it lets me mock away the full database layer and only test the api and service implementation instead.
