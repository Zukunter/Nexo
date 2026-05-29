# Commands

---

## -i | --input

- Meaning : Get the content of that file.
- Argument : An absolute or relative path. If it is a folder will connect each file in that directory and folder if cascading is bigger than 0.

## -o | --output

- Meaning : Where to write out all copy content.
- Argument : An absolute or relative path. Will always write out a file.

## -r | --recursive

- Meaning : If shoud handle calls from input's files. Will look for "prefix + nexus", all next in the line must be any of theese commands.
- Argument : A boolean value (true or false).

## -d | --debug

- Meaning : If shoud show debug message.
- Argument : A boolean value (true or false).

## -c | --cascading

- Meaning : How many content's folders shoud read in cascading.
- Argument : A range between 0 and 65 535.

## -p | --prefix

- Meaning : The prefix that is going to be look for.
- Argument : A valid string.

## -e | --execute

- Meaning : Execute a command.
- Argument : A value command with its args and with a final _ that tells nexus that the arguments have ended.
