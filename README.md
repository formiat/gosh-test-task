# Compressive merge

Program that merges (concatenates) given words into single string, that contains all of given words, and has minimum possible length.

## Run

`cargo run -- <params>`

Call example: `cargo run -- --words fear,leads,to,anger,anger,leads,to,hatred,hatred,leads,to,conflict,conflict,leads,to,suffering`

## CLI params

To get help info - pass `-h` (`cargo run -- -h`) param when call.

- **words** - Comma-separated list of words to be merged

## Word list examples

- `тачка,качалка,калач,лачуга,оплачу,чуйка,туловище,щелочь,каток,токамак,макинтош,картошка,гавкать,томат,киношка,тошкино,шкив,кивать,автомат,поварешки,тарантул,ладошки,плацкарт,остапов,шоколад,артишок,каунтач,автокар,тьмутаракань,канье,макавто,стерлитамак,хипстер`
- `fear,leads,to,anger,anger,leads,to,hatred,hatred,leads,to,conflict,conflict,leads,to,suffering`
- `сержант,схватил,автомат,Калашникова,упер,в,синий,живот,и,с,наслаждением,стал,стрелять,в,толпу,толпа,уперла,автомат,схватила,Калашникова,сержанта,и,стала,стрелять,с,наслаждением,в,синий,живот,Калашников,автомат,с,наслаждением,стал,стрелять,в,толпу,в,сержанта,в,живот,в,синeе`

## Algorithm complexity and memory requirement

Not estimated
