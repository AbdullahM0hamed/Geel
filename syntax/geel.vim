" Vim syntax file
" Language: Geel
" Maintainer: AbdullahM0hamed

if exists("b:current_syntax")
    finish
endif

syn keyword geelOne hadduu ama kastoo intuu jooji iyo isticmaal sii kujira tijaabo qabo sida gudub maaha kayd tir caalami waa Tus celi dhaaf xaqiiji laamda
syn keyword geelTwo Run Been Waxba qayb kulli midkasta labaale bool dhis qaamuus sifosheeg tiri qiimee bax kasooc tobanle hagaaji caalamiyaasha caawimaad lixyatobaneyn weydii tirodhan dherer uguweyn uguyar wad wax siddeedid fur qor sifo faraq muuqaal rogan tirobuuxin urur qaybi soocan qoraal iskudar uruur nooc iskuxer markuu kamidmid kawad qoraalkadhig bartaanbaar kooxdhibco dhibco mashaquuqabtaa qoraalmid kakan sifotir samee ururbadalmeyn sifokeen lambarugaar makaydkoosocotaa makaydkuudhaxlay xeradaan kushaqee xasuusaragti fur lambarkadhig dhufocelcelis sifobadal qaybguud doorsoomayaal
syn keyword geelThree ka keen
syn keyword geelFour KhaladAasaasi Khalad KhaladXisaabeed KhaladXaqiijin KhaladSifeed KhaladKeenid KhaladJagaale WaaLaJoojiyey KhaladXasuuseed KhaladMagac KhaladLamaSameyn KhaladCelcelis NoocKhaldan KhaladQiimeyn KhaladEberUQeybin KhaladOgolaansho Digniin DigniinKeenid KhaladXusaaaKuMeelGaar KhaladRaadin KhaladKaydDibadeedLamaHelin KhaladFuro KhaladHabdhis KhaladWeynaan KhaladTixraac KhaladGoortaShaqada KhaladBeegmid KhaladBoodid KhaladHabdhis KhaladMaJiro KhaladHabxarfeed KhaladXarfeedUBadal KhaladXarfeedKaBadal KhaladTurjumidHabxarfeed KhaladDibadeed KhaladGB KhaladGBHalHal KhaladHawlQabashoDhaxlo KhaladXiriir KhaladTuubboJaban KhaladDibUXiriir KhaladKaydWuuJira KhaladKaydLamaHelin KhaladLaGoo KhaladWaaGal KhaladGalMaaha KhaladHawlQabashoRaadin KhaladWaqtigaaKaDhamaaday DigniinShaqsi DigniinWaaDuug DigniinDuugBuuNoqon DigniinHabQoraal DigniinGoortaShaqada DigniinMustaqbal DigniinHabxarfeed DigniinBadalid DigniinDhibco DigniinHanti ShaqaaleNoqnoqodBax JoojiNoqnoqodka JoojiKalaNoqnoqodka HabdhisBax
syn match symbols "haddii-kale" "ugu-dambeyn"
syn match numbers /[0-9]/
syn match speech /".*"/
syn match comments /\/\/.*$/
syn region multiline start=/\/\*/ end=/\*\//

hi geelOne ctermfg=yellow
hi geelTwo ctermfg=cyan
hi geelThree ctermfg=darkcyan
hi geelFour ctermfg=lightgreen
hi symbols ctermfg=yellow
hi numbers ctermfg=lightgreen
hi speech ctermfg=magenta
hi comments ctermfg=lightcyan
hi multiline ctermfg=lightcyan
