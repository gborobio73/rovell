# rovell

![](https://github.com/gborobio73/rovell/blob/master/logo.png)

Aren't you _fatigat_ from writing Rust programs in English? Do you like saying
"collons" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Catalan touch to your
programs?

**rovell** (Catalan for _Rust_) is here to save your day, as it allows you to
write Rust programs in Catalan, using Catalan keywords, Catalan function names,
Catalan idioms.

This has been designed to be used as the official programming language to
develop the future Catalan sovereign operating system.

You're from Barcelona and don't feel at ease using only Catalan words? Don't worry!
Catalan Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

### Usage

Here's an example of what can be achieved with Rovell:

```rust
rovell::rovell! {
    funció principal() {
        deixa mutable x = 31;

        iguala x {
            42 => {
                collons!("mare de déu")
            }
            _ => escriulínia!("Com estàs!")
        }

        per i en 0..10 {
          escriulínia!(i)
        }

        merda!("f")
    }
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

More detailed examples [here](./rovell/examples/).

## Per què

* If the French can do it, so can we
* If the Spanish can do it, so can we

## Moltes gràcies

Big thanks to [Benjamin Bouvier](https://github.com/bnjbvr) for the original French
implementation, [Eliaz Bobadilla](https://github.com/UltiRequiem) for the Spanish inspiration, and also [Francesc](https://github.com/rfranr) for the Catalan review and wisdom.
## La llicència

Licensed under the MIT License.
