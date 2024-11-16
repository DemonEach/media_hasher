# media_hasher

This tool renames all files with the corresponding hashes.

By default it uses Sha224 and tries to find "media" ("jpg", "jpeg", "gif", "bmp", "png", "webp", "webm", "tiff", "mp4", "mpg", "mov") files in current directory, but you could provide specific files by "-f" flag. It does not check file signature, only file extension. 

```
Usage: ih [command] [options]  
Options:  
   -h, --help    - help information  
   -v, --verbose - print debug information  
   -f, --files    - provided list of files to proceed with renaming  
```