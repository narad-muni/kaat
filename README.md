# કાટ (Kaat)

![](logo.png)

Aren't you _કંટાળી ગયા_ from writing Rust programs in English? Do you like saying
"kaat" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Gujarati touch to your
programs?

**કાટ (Kaat)** (Gujarati for _Rust_) is here to save your day, as it allows you to
write Rust programs in Gujarati, using Gujaratikeywords, Gujarati function names.

You're from Gujarat and don't feel at ease using only English words? Don't worry!
Gujarati Rust is fully compatible with English-Rust, so you can mix both at your
convenience. Support for Gujarati Rust is not yet available.

Here's an example of what can be achieved with Roest:

```rust
    gebruik std::collections::Woordenboek zoals Wbk;

    karaktereigenschap SleutelWaarde {
        functie schrijf(&zelf, sleutel: Keten, waarde: Keten);
        functie lees(&zelf, sleutel: Keten) -> Mogelijkheid<&Keten>;
    }

    vast veranderlijk WOORDENBOEK: Mogelijkheid<Wbk<Keten, Keten>> = Geen;

    structuur Concreet;

    uitwerking SleutelWaarde voor Concreet {
        functie schrijf(&zelf, sleutel: Keten, waarde: Keten) {
            laat wk = gevaarlijk {
                WOORDENBOEK.verkrijg_of_voeg_toe_met(Standaard::standaard)
            };
            wk.voeg_in(sleutel, waarde);
        }
        functie lees(&zelf, sleutel: Keten) -> Mogelijkheid<&Keten> {
            laat wk = gevaarlijk {
                WOORDENBOEK.verkrijg_of_voeg_toe_met(Standaard::standaard)
            };
            wk.verkrijg(&sleutel)
        }
    }
```

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.that's it.

## contributies

First of all, _આભાર_ for considering participating to this joke,
Narendra Modi will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `મુખ્ય` (Gujarati for
`main`) branch.

## but why would you do dat

- આનંદ માટે (For fun)
