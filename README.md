# Gallerist

Discord bot built with Tokio and Serenity in Rust to display the object of the day from the **SAINT LOUIS ART MUSEUM**

ref : <https://www.slam.org/explore-the-collection/object-of-the-day/>

```

Workflow

1. bot to listen for command
2. deploy webscrapper to ref site
    2.1 Download image locally
    2.2 Formulate the description and metadata details regarding the object
3. and return the object of the day *with details* (optional)

```

## Important Refrences + Notes:

1. Serenity examples useful ref: <https://github.com/serenity-rs/serenity/tree/current/examples>

2. Tokio used for async -- need to completely understand

3. YT video series used as ref: <https://www.youtube.com/watch?v=tl766BHuAbE&list=PLzIwronG0sE5lQCPFP69Ukgz4d9dngaSi&index=10>

4. understand the use of std::env, namely env variables and how to now hardcode the token

5. Understand traits used for funcs

6. Notes on commands ref: <https://betterprogramming.pub/writing-a-discord-bot-in-rust-2d0e50869f64>
