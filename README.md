<p align="center">
  <img src="https://user-images.githubusercontent.com/29304787/157297344-a2e8d0d5-27f1-4c5c-9ae5-d5f48c2ada09.png" />
  Generate fake data for subscription forms
</p>

Faker is a small command line tool designed to generate all the random data you'll ever need for a service subscription.
Each command run will produce new randomic values chosen from a wide pool provided by the `fake-rs` library.

The executable accepts no parameters by default. A single parameter can be supplied to selectively toggle the generated fields.
This is achieved in a somewhat similar fashion to the unix `tar` utility:
```
Usage: faker flags
The `flags` parameter is a sequence of characters each toggling the generation of a field
By default its value is `flbpt`
    f 		Generate a first name (default: true)
    l 		Generate a last name (default: true)
    b 		Generate a birth date (default: true)
    p 		Generate a birth place (default: true)
    t 		Generate a phone number (default: true)
    s 		Generate a shipping location (default: false)
```

If any more random fields are needed, don't hesitate to open an issue.
