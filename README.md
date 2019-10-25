# Wellesley High School Discord Help Server Administration Bot
WHS Help Bot for short

# Table of contents
* [Tutorial](#Tutorial)
    * [Using The Bot](#Using-The-Bot)
        * [Command Syntax](#Command-Syntax)
        * [The Help Command](#The-Help-Command)
    * [Student Commands](#Student-Commands)
        * [Joining Classes](#Joining-Classes)
        * [Leaving Classes](#Leaving-Classes)
        * [Viewing Your Classes](#Viewing-Your-Classes)
        * [Viewing All Classes](#Viewing-All-Classes)
    * [Admin Commands](#Admin-Commands)
* [FAQ](#FAQ)
* [Support](#Support)
* [License](#License)

# Tutorial
This tutorial is aimed to get people started in using the WHS help bot.
Please read each section carefully and feel free to submit a pull request
[here](https://github.com/DusterTheFirst/WHS-Help-Bot/edit/master/README.md)
if you have the ability to do so and have found a mistake in the documentation.

With that out of the way let us get started.

## Using The Bot
To start using the bot, the first thing you need to understand is how to send
commands.

### Command Syntax
The syntax to send a command is:

```<prefix><command> [..subcommand] [..options]```

If this makes no sense to you, let me break it down. This type of syntax will be used much
more throughout the tutorial.

Anything in these "triangular brakets" (`<>`) is **required**.
The text inside of them tell you what would go in place of them.
On the other hand, anything surrounded in "square brackets" (`[]`) is optional.
The `..` in front of a placeholder tells you that there can me one or more of
them, as you will see later in the tutorial.

As an example, let us say we have a bot with a prefix `!` and we are trying to run the command `help`.
To run this command we would type `!help`. Let us break this apart, The `!` in this case went in place of
the `<prefix>` placeholder and the `help` where the `<command>` was. The command has no subcommands nor
arguments so we can ignore those.

That was simple right? Well lets do another. We now have the sub command
`join` in the category `classes`. This command also takes a single argument
`class`. In order to run this command, first we need to write it out.
To start out, you write the prefix `!`, next you can add the command (or categoy)
name. So far we have:

    !classes

Next we need ot add the subcommand, `join`.
In order to tell the bot that we are running a subcommand, we need to add a space and then
the name leving us with:

    !classes join

If we were to run this command right now, it would give us an error:

    This command requires 1 argument(s) while it was only given 0

This would give an inkling of what we forgot, the class we want to join. To add the class,
we can just put a space and the class we want to join, for example `ACP History`. Doing so
would leave us with:

    !classes join ACP History

But this would **not** work. We would be given a different error this time:

    The command can only take up to 1 argument(s), but was given 2 argument(s).

This tells us that we gave it 2 arguments, but we only gave it one, didnt we? Well not exactly,
the bot is not smart enough to tell that `ACP History` is one argument it sees it as 2: `ACP` and
`History`. In order to tell it that it is one argument just with a space in it we can surround it
in qoutation marks (`""`). Our last and final iteration of the command would be

    !classes join "ACP History"

At this point, if you were to send this message, the bot would add you into the `ACP History` class
and you would be all done.

If you wanted to know more about another command, or to understand its syntax, you can do so with the
help command.

### Using The Help Command


# Support
If you have found an issue with the bot, please report it to @DusterTheFirst
on discord or if you have a GitHub accont, please file an issue
[here](https://github.com/DusterTheFirst/WHS-Help-Bot/issues).

If you need assistance in using the bot, feel free to ask @DusterTheFirst or other
moderators in the server

# License
        A bot designed for the WHS help server
        Copyright (C) 2019  Zachary Kohnen

        This program is free software: you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation, either version 3 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program.  If not, see <https://www.gnu.org/licenses/>.