<div align="center">
<img src="https://github.com/AbdullahM0hamed/Geel/blob/master/logo.png">
<br>
<strong><i>An interpreted, python-inspired, Somali programming language written in Rust</i></strong>
</div>

# Build

To build the compiler, simply do: `cargo build` in the root folder.

# Usage

To use the REPL run: `target/debug/geel`

To run code from a file run `target/debug/geel -k [FILE]`

To run code from a string run `target/debug/geel -q [STRING]`

# Introduction

This progamming language currently has basic support for if statements (but no support for nesting yet, also only currently supports comparisons and not return values from functions), variables, and calculations. Support for other features are planned.

Some basic supported code:

## Variable assignment

```
>>> a = 5
>>> a
5
```

## Inbuilt function (print)

```
>>> qor("Some text")
"Some Text"
```

## If Statement

```
>>> a = 5
>>> hadduu a == 5:
...   qor("Some Text")
...
"Some Text"
```

Below is a checklist of keywords, methods, and errors that are planned, as well as their status - on the left will be a python keyword (except the list which is kotlin) and on the right will be the Somali word I've selected for it:

## Keywords

[x] - and -> iyo
[ ] - not -> maaha
[x] - or -> ama
[ ] - continue -> gudub
[ ] - boolean -> booliyan
[ ] - break -> jooji
[ ] - True -> Run
[ ] - False -> Been
[ ] - None -> Waxba
[ ] - import -> keen
[ ] - from -> ka
[ ] - as -> sida

### Planned usage for imports

```
>>> [LIBRARY] ka keen [PACKAGE]
>>> [LIBRARY] ka keen [PACKAGE] sida [NAME]
>>> [LIBRARY] keen
```

[ ] - try -> tijaabi
[ ] - except -> qabo
[ ] - finally -> ugu-dambeyn
[ ] - assert -> xaqiiji
[ ] - class -> kayd
[ ] - def -> qayb
[ ] - del -> tir
[x] - if -> hadduu
[x] - elif -> ama hadduu
[x] - else -> haddii-kale
[ ] - for -> kastoo
[ ] - in -> kujira

### Planned usage for for loops

```
>>> x kastoo kujira [1,2,3,4,5]:
...   [CODE]
...
```

[ ] - global -> caalami
[ ] - is -> waa
[ ] - lambda -> laamda
[ ] - nonlocal -
[ ] - pass -> dhaaf
[ ] - raise -> tus
[ ] - return - celi
[ ] - while -> intuu
[ ] - with -> isticmaal
[ ] - yield -> sii

## Built-in functions

[ ] abs -> qiimahasugan
[ ] aiter ->
[ ] all -> kulli
[ ] anext ->
[ ] any -> midkasta
[ ] ascii ->
[x] bin -> labaale
[ ] bool -> bool
[ ] breakpoint ->
[ ] bytearray ->
[ ] bytes ->
[ ] callable ->
[ ] chr ->
[ ] classmethod -> qaybkaydeed
[ ] compile -> dhis
[ ] complex ->
[ ] delattr ->
[ ] dict -> qaamuus
[ ] dir -> sifosheeg
[x] divmod -> qaybiyobaaq
[ ] enumerate -> tiri
[ ] eval -> qiimee
[ ] exec ->
[ ] exit -> bax
[ ] filter -> kasooc
[ ] float -> tobanle
[ ] format -> hagaaji
[ ] frozenset ->
[ ] getattr ->
[ ] globals -> caalamiyaasha
[ ] hasattr -> sifomaleeyahay
[ ] hash ->
[ ] help -> caawimaad
[ ] hex -> lixyatobaneyn
[ ] id -> lambarugaar
[ ] input -> weydii
[ ] int -> tirodhan
[ ] isinstance ->
[ ] issubclass ->
[ ] iter -> midmid
[ ] len -> dherer
[ ] list -> aruur
[ ] locals ->
[ ] map ->
[ ] max -> ugubadnaan
[ ] memoryview ->
[ ] min -> uguyaraan
[ ] next -> wad
[ ] object -> wax
[ ] oct -> siddeedid
[ ] open -> fur
[ ] ord ->
[ ] pow ->
[x] print -> qor
[ ] property -> sifo
[x] range -> faraq
[ ] repr -> muuqaal
[ ] reversed -> rogan
[ ] round -> tirobuuxin
[ ] set -> urur
[ ] setattr ->
[ ] slice -> qaybi
[ ] sorted -> soocan
[ ] staticmethod ->
[ ] str -> qoraal
[ ] sum -> iskudar
[ ] super -> dhaxal
[ ] tuple -> uruur
[ ] type -> nooc
[ ] vars ->
[ ] zip -> iskuxer

## Exceptions

[ ] BaseException -> KhaladAasaasi
[ ] Exception -> Khalad
[ ] ArithmeticError -> KhaladXisaabeed
[ ] BufferError ->
[ ] LookupError ->
[ ] AssertionError -> KhaladXaqiijin
[ ] AttributeError -> KhaladSifeed
[ ] EOFError -> KhaladQoraalDhamaa
[ ] FloatingPointError -> KhaladTobanle
[ ] GeneratorExit ->
[ ] ImportError -> KhaladKeenid
[ ] ModuleNotFoundError ->
[ ] IndexError -> KhaladJagaale
[ ] KeyError ->
[x] KeyboardInterrupt -> WaaLaJoojiyey
[ ] MemoryError -> KhaladXasuuseed
[ ] NameError -> KhaladMagceed
[ ] NotImplementedError -> KhaladLamaSameyn
[ ] OSError ->
[ ] OverflowError ->
[ ] RecursionError -> KhaladCelcelis
[ ] ReferenceError ->
[ ] RuntimeError ->
[ ] StopIteration ->
[ ] StopAsyncIteration ->
[ ] IndentationError ->
[ ] TabError ->
[ ] SystemError ->
[ ] SystemExit ->
[ ] TypeError -> NoocKhaldan
[ ] UnboundLocalError ->
[ ] UnicodeError ->
[ ] UnicodeEncodeError ->
[ ] UnicodeDecodeError ->
[ ] UnicodeTranslateError ->
[ ] ValueError -> KhaladQiimeyn
[ ] ZeroDivisionError -> KhaladEberUQeybin
[ ] EnvironmentError ->
[ ] IOError ->
[ ] WindowsError ->
[ ] BlockingIOError ->
[ ] ChildProcessError ->
[ ] ConnectionError ->
[ ] BrokenPipeError ->
[ ] ConnectionAbortedError -> KhaladXiriirLaGoo
[ ] ConnectionRefusedError -> KhaladXiriirLaDiid
[ ] ConnectionResetError ->
[ ] FileExistsError ->
[ ] FileNotFoundError ->
[ ] InterruptedError ->
[ ] IsADirectoryError ->
[ ] NotADirectoryError ->
[ ] PermissionError -> KhaladOgolaansho
[ ] ProcessLookupError ->
[ ] TimeoutError ->
[ ] Warning -> Digniin
[ ] UserWarning ->
[ ] DeprecationWarning ->
[ ] PendingDeprecationWarning ->
[ ] SyntaxWarning ->
[ ] RuntimeWarning ->
[ ] FutureWarning ->
[ ] ImportWarning -> DigniinKeenid
[ ] UnicodeWarning ->
[ ] EncodingWarning ->
[ ] BytesWarning ->
[ ] ResourceWarning ->

## Additional (extra-pythonic)

[ ] - when -> markuu
