# bsmv

Make BeatMap filenames readable again

## Usage

Copy `bsmv.exe` to your custom songs directory and run it.

## Advanced usage

Optionally, you can choose a custom rename pattern using the following placeholders

* `%N` Song Name
* `%n` Song Subname
* `%A` Song Author Name
* `%a` Creator of this Beatmap

```powershell
> .\bsmv.exe [%a] %A - %N
```
