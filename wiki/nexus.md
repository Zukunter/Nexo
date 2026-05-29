# Nexus file

It is a file where nexus won't look for "prefix + nexus", instaed it will be nothing. So you can directly write arguments line per line

It is the standar input if -i input is not set. So you can simple write this and will look for ./nexus

```bash
nexus
```

---

## Example

```bash
-o ../Output/file1 -i ./file1 -r true -c 600 -p //
-o ../Output/file2 -i ./file2 -r true -c 600 -p //
-o ../Output/file4 -i ./foler4 -r true -c 600 -p --
-o ../Output/file3 -i ./file3 -r true -c 600 -p #
-o ../Output/file5 -i ./file5 -r true -c 600 -p //
```
