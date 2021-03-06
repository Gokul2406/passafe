---
title: Passafe
---

# Passafe The password manager for \*nix Operating Systems

Passafe is a password manager made for \*nix using rust Passafe stores
the passwords in your local filesystem. This ensures that only you the
user will be able to see the passwords Fun thing about passafe is that
it doesn\'t store passwords in raw format rather it encrypts it using
the master password provided by the user

# Why should you use a password manager

-   A password manager stores all the passwords that you give it and
    reduces your hassle for remembering passwords in a large way. By
    using a password manager you only need to remember one password to
    access all your passwords.
-   What passafe does that is great for you is that it stores your
    passwords in your filesystem

# Why should you use passafe

-   Passafe stores all your passwords in your filesystem which means
    that the password that you create will only be accessible to you and
    to those whi knows the master password for passafe
-   By doing this we are also giving complete freedom for our users. We
    are not a free as in no fee software we are a **Free as in freedom
    software**

# FAQs about passafe

-   Where are passwords stored ?

    Passwords are stored in **\~/.local/share/passafe/passwords.json**
    file JSON is easily readable but since passafe stores passwords by
    encrypting it no one can look at the password by using the file

-   Is passafe collecting any data ?

    No not at all. Passafe doesn\'t collect any info or store your
    password in a database. If you want to check whether we are telling
    the truth it\'s easy just read to source code and you will
    understand that passafe doesn\'t store any of your data in a
    database

# Installation

## Compiling from source

-   Clone the repo using

``` {.bash}
git clone https://github.com/Gokul2406/passafe.git
```

-   After cloning the repo cd into passafe

``` {.bash}
cargo install --path .
```

## Binary installation

-   You can download the binary from
    <https://github.com/Gokul2406/passafe/releases/download/1.0/passafe>

-   Add the file into your path

-   Run the program using

    ``` {.bash}
    passafe
    ```

# Features implemented so far

Creating a master password

Storing a master password

Login functionality

Creating Passwords and storing them

Encrypting user inputed passwords before storing them

Printing all the passwords given by the user after decrypting it

Coloured outputs

# Contributing

This project is open to contributions from anyone.

# Issues

If you find any bugs or have a feature you want create an issue it will
really help the project grow

# Contact The Author

## Email

gokulpbharathan89\@gmail.com
