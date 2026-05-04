# media_hasher

This tool renames all files with the corresponding hashes.

By default it uses Sha224 and tries to find "media" ("jpg", "jpeg", "gif", "bmp", "png", "webp", "webm", "tiff", "mp4", "mpg", "mov") files in current directory, but you could provide specific files by "-f" flag. It does not check file signature, only file extension. 
You can override the searched extensions with "-e" or "--extensions".

```
Usage: ih [command] [options]  
Options:  
   -h, --help    - help information  
   -v, --verbose - print debug information  
   -f, --files    - provided list of files to proceed with renaming  
   -e, --extensions - provided list of file extensions to search for  
```
