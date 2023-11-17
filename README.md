<div align="center">
<img src="https://github.com/AbdullahM0hamed/Geel/blob/master/logo.png">
<br>
<strong><i>An interpreted, python-inspired, Somali programming language written in Rust</i></strong>
</div>

# Build

To build the interpreter, simply do: `cargo build` in the root folder.

# Demo

The REPL can be used here: [geel](https://replit.com/@Red0192/Geel)

# Examples

The examples folder contains some simple sample code written in the Geel language. Below are the examples, and whether they are currently supported (a demo of supported examples being run can be found [here](https://replit.com/@Red0192/Geel-Examples):

- [x] Fizzbuzz example

- [ ] Fibonacci example

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

### For loops

```
>>> x kastoo faraq(1,10) kujira:
...   qor("Some Text")
...
```

Below is a checklist of keywords, methods, and errors that are planned, as well as their status - on the left will be a python keyword (except the last which is kotlin) and on the right will be the Somali word I've selected for it:

## Keywords

- [x] and -> iyo

- [x] not -> aheyn

- [x] or -> ama

- [x] continue -> gudub

- [ ] boolean -> booliyan

- [x] break -> jooji

- [x] True -> Run

- [x] False -> Been

- [x] None -> Waxba

- [ ] import -> keen

- [ ] from -> ka

- [ ] as -> sida


### Planned usage for imports

```
>>> [LIBRARY] ka keen [PACKAGE]
>>> [LIBRARY] ka keen [PACKAGE] sida [NAME]
>>> [LIBRARY] keen
```

- [ ] try -> tijaabi

- [ ] except -> qabo

- [ ] finally -> ugu-dambeyn

- [ ] assert -> xaqiiji

- [ ] class -> kayd

- [ ] def -> qayb

- [x] del -> tir

- [x] if -> hadduu

- [x] elif -> ama hadduu

- [x] else -> haddii-kale

- [x] for -> kastoo

- [x] in -> kujira

- [ ] global -> caalami

- [ ] is -> waa

- [ ] lambda -> laamda

- [ ] nonlocal -> xerokale

- [ ] pass -> dhaaf

- [ ] raise -> tus

- [ ] return - celi

- [ ] while -> intuu

- [ ] with -> isticmaal

- [ ] yield -> sii


## Built-in functions

- [ ] abs -> qiimahasugan

- [ ] aiter -> kamidmid

- [ ] all -> kulli

- [ ] anext -> kawad

- [ ] any -> midkasta

- [ ] ascii -> qoraalkadhig

- [x] bin -> labaale

- [ ] bool -> bool

- [ ] breakpoint -> bartaanbaar

- [ ] bytearray -> kooxdhibco

- [ ] bytes -> dhibco

- [ ] callable -> mashaquuqabtaa

- [ ] chr -> qoraalmid

- [ ] classmethod -> qaybkaydeed

- [ ] compile -> dhis

- [ ] complex -> kakan

- [ ] delattr -> sifotir

- [ ] dict -> qaamuus

- [ ] dir -> sifosheeg

- [x] divmod -> qaybiyobaaq

- [ ] enumerate -> tiri

- [ ] eval -> qiimee

- [ ] exec -> samee

- [x] exit -> bax

- [ ] filter -> kasooc

- [ ] float -> tobanle

- [ ] format -> hagaaji

- [ ] frozenset -> ururbadalmeyn

- [ ] getattr -> sifokeen

- [ ] globals -> caalamiyaasha

- [ ] hasattr -> sifomaleeyahay

- [ ] hash -> lambarugaar

- [ ] help -> caawimaad

- [ ] hex -> lixyatobaneyn

- [ ] id -> lambarlagugarto

- [ ] input -> weydii

- [x] int -> tirodhan

- [ ] isinstance -> makaydkoosocotaa

- [ ] issubclass -> makaydkuudhaxlay

- [ ] iter -> midmid

- [ ] len -> dherer

- [ ] list -> aruur

- [ ] locals -> xeradaan

- [ ] map -> kushaqee

- [ ] max -> ugubadnaan

- [ ] memoryview -> xasuusaragti

- [ ] min -> uguyaraan

- [ ] next -> wad

- [ ] object -> wax

- [ ] oct -> siddeedid

- [ ] open -> fur

- [ ] ord -> lambarkadhig

- [ ] pow -> dhufocelcelis

- [x] print -> qor

- [ ] property -> sifo

- [x] range -> faraq

- [ ] repr -> muuqaal

- [ ] reversed -> rogan

- [ ] round -> tirobuuxin

- [ ] set -> urur

- [ ] setattr -> sifobadal

- [ ] slice -> qaybi

- [ ] sorted -> soocan

- [ ] staticmethod -> qaybguud

- [ ] str -> qoraal

- [ ] sum -> iskudar

- [ ] super -> dhaxal

- [ ] tuple -> uruur

- [ ] type -> nooc

- [ ] vars -> doorsoomayaal

- [ ] zip -> iskuxer


## Exceptions

- [ ] BaseException -> KhaladAasaasi

- [ ] Exception -> Khalad

- [ ] ArithmeticError -> KhaladXisaabeed

- [ ] BufferError -> KhaladXasuusKuMeelGaar

- [ ] LookupError -> KhaladRaadin

- [ ] AssertionError -> KhaladXaqiijin

- [ ] AttributeError -> KhaladSifeed

- [ ] EOFError -> KhaladQoraalDhamaa

- [ ] FloatingPointError -> KhaladTobanle

- [ ] GeneratorExit -> ShaqaaleNoqnoqodBax

- [ ] ImportError -> KhaladKeenid

- [ ] ModuleNotFoundError -> KhaladKaydDibadeedLamaHelin

- [ ] IndexError -> KhaladJagaale

- [ ] KeyError -> KhaladFuro

- [x] KeyboardInterrupt -> WaaLaJoojiyey

- [ ] MemoryError -> KhaladXasuuseed

- [ ] NameError -> KhaladMagceed

- [ ] NotImplementedError -> KhaladLamaSameyn

- [ ] OverflowError -> KhaladWeynaan

- [ ] RecursionError -> KhaladCelcelis

- [ ] ReferenceError -> KhaladTixraac

- [ ] RuntimeError -> KhaladGoortaShaqada

- [ ] StopIteration -> JoojiNoqnoqodka

- [ ] StopAsyncIteration -> JoojiKalaNoqnoqodka

- [ ] IndentationError -> KhaladBeegmid

- [ ] TabError -> KhaladBoodid

- [ ] SystemError -> KhaladHabdhis

- [ ] SystemExit -> HabdhisBax

- [ ] TypeError -> NoocKhaldan

- [ ] UnboundLocalError -> KhaladMaJiro

- [ ] UnicodeError -> KhaladHabxarfeed

- [ ] UnicodeEncodeError -> KhaladHabxarfeedUBadal

- [ ] UnicodeDecodeError -> KhaladHabxarfeedKaBadal

- [ ] UnicodeTranslateError -> KhaladTurjumidHabxarfeed

- [ ] ValueError -> KhaladQiimeyn

- [ ] ZeroDivisionError -> KhaladEberUQeybin

- [ ] EnvironmentError -> KhaladDibadeed

- [ ] IOError -> KhaladGB (Galin-Bixin)

- [ ] BlockingIOError -> KhaladGBHalHal

- [ ] ChildProcessError -> KhaladHawlQabashoDhaxlan

- [ ] ConnectionError -> KhaladXiriir

- [ ] BrokenPipeError -> KhaladTuubboJaban

- [ ] ConnectionAbortedError -> KhaladXiriirLaGoo

- [ ] ConnectionRefusedError -> KhaladXiriirLaDiid

- [ ] ConnectionResetError -> KhaladDibUXiriir

- [ ] FileExistsError -> KhaladKaydWuuJiraa

- [ ] FileNotFoundError -> KhaladKaydLamaHelin

- [ ] InterruptedError -> KhaladLaGoo

- [ ] IsADirectoryError -> KhaladWaaGal

- [ ] NotADirectoryError -> KhaladGalMaaha

- [ ] PermissionError -> KhaladOgolaansho

- [ ] ProcessLookupError -> KhaladHawlQabashoRaadin

- [ ] TimeoutError -> KhaladWaqtigaaKaDhamaaday

- [ ] Warning -> Digniin

- [ ] UserWarning -> DigniinShaqsi

- [ ] DeprecationWarning -> DigniinWaaDuug

- [ ] PendingDeprecationWarning -> DigniinDuugBuuNoqon

- [ ] SyntaxWarning -> DigniinHabQoraal

- [ ] RuntimeWarning -> DigniinGoortaShaqada

- [ ] FutureWarning -> DigniinMustaqbal

- [ ] ImportWarning -> DigniinKeenid

- [ ] UnicodeWarning -> DigniinHabxarfeed

- [ ] EncodingWarning -> DigniinBadalid

- [ ] BytesWarning -> DigniinDhibco

- [ ] ResourceWarning -> DigniinHanti


## Additional (extra-pythonic)

- [ ] when -> markuu

