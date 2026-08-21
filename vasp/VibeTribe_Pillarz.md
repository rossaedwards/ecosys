# Vibe Tribe Pillarz

**VASP 3.69** · TSLCA 3² lattice · 9 pillars × 13 questions  
**Aurphyx LLC** · Vibe Audio Player / Vibe Media Player  
**Updated:** 2026-08-21

A Like / Fav / Love is one bit. The VASP button is nine rooms. Click a room and the Tribe answers questions that actually fill that pillar — personal, blunt, no museum voice.

Machine harvest fills IDENTITY and any field with evidence. The Tribe fills what catalogs cannot: how it hits the body, the room, the bed, the booth. Unanswered stays `unknown`. Votes never invent a `known` DSP number.

---

## Interface

```
[ now playing ]
        [ VASP ]
   +---+---+---+
   | 1 | 2 | 3 |   1 Structural   2 Tonal     3 Timbral
   +---+---+---+   4 Linguistic   5 Affective 6 Contextual
   | 4 | 5 | 6 |   7 Photometric  8 Kinetic   9 Genealogical
   +---+---+---+
   | 7 | 8 | 9 |
   +---+---+---+
```

Tap a cell → that pillar’s page. Thirteen questions. Each question is one schema leaf (or a tight pair). Choices A–D map to enums / numbers. **Skip** writes `null` and tags `tribe` pending. Submit writes one vote into the running Tribe aggregate. You can leave after one question. Completing all 13 on one pillar is a full cell.

Q01 on every pillar is the gut hit — same energy as: *How would you dance with a sexy person of interest to this??*

Q02–Q13 walk the three official sub-objects for that pillar (four-ish leaves each, plus one closer).

---

## Harvest first, vote second

Baselines before anyone opens a cell. Credits and catalogs are not lyrics and they are not valence.

| Source | What it is allowed to fill |
| --- | --- |
| Filename / ID3 | `IDENTITY.ARTIST`, `TITLE`, album, `TBPM` → `BPM_RAW` if tagged, `TKEY`, `TCON` |
| TIDAL credits / track | identity, ISRC, performers, writers, engineers → Genealogical + Timbral production notes |
| MusicBrainz | MBID, duration, release date → `ERA_ANCHORING.RELEASE_DATE`, work IDs |
| Discogs | label, catalog no., matrix, pressing → `DNA_SAMPLING`, lineage |
| Billboard / official charts | peak, weeks → Contextual history only, not “mood” |
| Beatport / Traxsource | EDM BPM, Camelot/key, DJ genre → `BPM_RAW`, `KEY_SIGNATURE`, `GENRE_TREE` |
| 1001Tracklists | festival / set context → `MACRO_SETTING` candidates |
| SoundCloud / community comments | raw Tribe signal — never auto-`known` |
| Wikipedia | **song page only**. Artist bios stay out of pillars |
| DSP / scoring engine | `BPM_RAW`, centroid, LRA, Thayer from audio — highest rank for numbers |

Dick Clark’s vault is split across estates, Syracuse, and the Library of Congress. We do not have a dump of it. The Tribe *is* the living archive: same instinct as American Bandstand cards, except every listener can vote a pillar instead of mailing a postcard.

### Rank when sources fight

1. DSP / ID3 measured number  
2. Official catalog credit (TIDAL, MB, Discogs, Beatport)  
3. Tribe aggregate once `n` ≥ 13  
4. Single vote = `pending`  
5. Skip / silence = `unknown`

Tribe never overwrites a measured `BPM_RAW`. Tribe *does* own `BPM_PERCEIVED`, dance MET, color, scene, authenticity.

Vote record (sidecar, not a secret):

```json
{
  "voter": "soul_or_anon",
  "pillar": "KINETIC",
  "q": "P8-Q01",
  "choice": "A",
  "maps_to": "PILLARS.KINETIC.ENERGY_EXPENDITURE.MET_SCORE",
  "value": 2.8,
  "at": "2026-08-21T08:00:00Z"
}
```

Aggregate = running mean for numbers, mode for enums, conservative max for `EXPLICIT_FILTER`.

---

## Pillar 1 — STRUCTURAL · The Skeleton

Harvest: Beatport BPM, ID3 `TBPM`, MusicBrainz duration, DSP transients.

**P1-Q01** Does the beat drop like a brick, punch you in the chest, or creep up your spine??  
→ `PERCUSSIVE_DNA.KICK_TRANSIENT.ATTACK` + `ARRANGEMENT_ARCHITECTURE.BREAKDOWN_DEPTH`  
A brick / instant · B chest punch · C slow creep · D never really drops

**P1-Q02** If you tapped your foot with your eyes closed, how fast is this actually moving??  
→ `TEMPORAL_DYNAMICS.BPM_PERCEIVED`  
A dragging / half-time · B walking pulse · C running · D double-time blur

**P1-Q03** Machine-locked to the grid, or does it stumble like a human??  
→ `GROOVE_QUANTIZATION`  
A bolted to the grid · B tight but breathing · C swung / late · D drunk on purpose

**P1-Q04** Can you count this in 4 without getting lost??  
→ `TIME_SIGNATURE`  
A straight 4/4 · B waltz / 3 · C odd (5, 7, 11…) · D I cannot find 1

**P1-Q05** When the drop or chorus hits, do you see it coming a mile away??  
→ `SECTIONAL_MARKERS`  
A telegraphed · B fair warning · C sucker punch · D no sections, just a slab

**P1-Q06** How many bars could a DJ mix on without it falling apart??  
→ `MIX_WINDOW_INDEX`  
A intro is a runway · B a few bars · C tight / radio · D don’t even try

**P1-Q07** In the breakdown, how far does the floor fall out??  
→ `BREAKDOWN_DEPTH`  
A still slamming · B knees bent · C almost silence · D no breakdown

**P1-Q08** That kick: click on the front, or a dull thud in your sternum??  
→ `KICK_TRANSIENT.ATTACK`  
A sharp click · B punch · C soft thud · D no kick worth naming

**P1-Q09** Does the kick die fast or does an 808 sit there and bloom??  
→ `KICK_TRANSIENT.DECAY`  
A tight / short · B medium · C long boom · D sub that never leaves

**P1-Q10** Kick personality in one word.  
→ `KICK_TRANSIENT.PROFILE`  
A click · B thud · C 808 · D industrial smash

**P1-Q11** Is the groove on the beat or teasing you off it??  
→ `SYNCOPATION_INDEX` (0–1)  
A on-the-one (0.1) · B some skip (0.35) · C heavily off (0.7) · D polyrhythm soup (0.95)

**P1-Q12** Those little ghost hits between the hits — busy or sparse??  
→ `GHOST_NOTE_DENSITY`  
A none · B a few winks · C busy pocket · D a whole second kit whispering

**P1-Q13** Would you trust this to hold a whole night without the structure getting boring??  
→ closer / arrangement confidence  
A built like a cathedral · B solid club tool · C three-minute sugar · D it already looped itself to death

---

## Pillar 2 — TONAL · The Flesh

Harvest: Beatport Camelot, ID3 `TKEY`, DSP key. Tribe owns how it *feels*.

**P2-Q01** Does the melody feel smooth and beautiful, or dark, tense, and like it knows something??  
→ `DISSONANCE_RATING`  
A pretty / resolved · B bittersweet · C tense / mysterious · D ugly on purpose

**P2-Q02** Major sunlight or minor basement??  
→ `KEY_SIGNATURE` (mode half)  
A major · B minor · C modal / neither · D I hear both fighting

**P2-Q03** Simple three-note honesty, or jazz chords stacking secrets??  
→ `CHORD_COMPLEXITY`  
A triads · B 7ths · C 9/11/13 soup · D clusters / no chords

**P2-Q04** How often does it scrape a wrong-on-purpose interval??  
→ `DISSONANCE_RATING`  
A almost never (0.1) · B spice (0.35) · C tritones living here (0.7) · D atonal scrape (0.95)

**P2-Q05** Does the tune stay in a small cage or jump octaves??  
→ `RANGE_SPAN`  
A tight · B a comfortable octave · C wide · D acrobat

**P2-Q06** Will this hook still be in your head in the shower tomorrow??  
→ `HOOK_STRENGTH`  
A already stuck · B catchy · C grows on you · D no hook, just weather

**P2-Q07** Does the melody walk step by step or leap like it’s late??  
→ `MELODIC_MOTION`  
A conjunct / steps · B mix · C big leaps · D spoken / no melody

**P2-Q08** Bright-concert pitch or something older / weirder??  
→ `REFERENCE_PITCH`  
A 440 world · B a hair flat / warm · C a hair sharp / bright · D not even in that game

**P2-Q09** Any notes that don’t live on a piano??  
→ `MICROTONALITY`  
A no · B bends / slides · C quarter-tone world · D I have no idea

**P2-Q10** If you hummed this to someone you wanted, would they get the hint??  
→ hook + contour  
A instantly · B after the chorus · C only if they already love you · D this is not a love letter

**P2-Q11** Does it resolve, or leave you hanging like a text on read??  
→ cadence feel → feeds Affective `RESOLUTION_STATE` later  
A lands home · B almost · C cliffhanger · D refuses to cadence

**P2-Q12** Harmony denser as it goes, or one idea the whole time??  
→ complexity over time  
A one loop · B verse/chorus contrast · C keeps adding · D falls apart on purpose

**P2-Q13** Would a DJ in a dark booth mix this on key without thinking??  
→ Camelot usefulness  
A yes, tool track · B if they care · C key is a suggestion · D atonal / don’t

---

## Pillar 3 — TIMBRAL · The Skin

Harvest: TIDAL engineers / mix credits, Discogs pressing, DSP centroid / LRA.

**P3-Q01** Clean and airy, warm like vinyl on a body, or raw and gritty??  
→ `TEXTURE_GRAIN.SURFACE` + `FIDELITY_SCORE`  
A airy glass · B warm vinyl · C raw grit · D blown-out wreck

**P3-Q02** Where does the weight sit — under the floor, in your face, or in the air??  
→ `FREQUENCY_BALANCE`  
A sub-dominant · B mid-forward · C air / brilliance · D scooped / no center

**P3-Q03** Sine-clean or square-wave dirty??  
→ `SPECTRAL_SATURATION`  
A clean · B lightly saturated · C thick harmonics · D crushed

**P3-Q04** Does it feel dark-brown or ice-bright??  
→ `SPECTRAL_CENTROID`  
A dark · B balanced · C bright · D brittle / harsh

**P3-Q05** Bedroom demo or mastering-suite flex??  
→ `FIDELITY_SCORE`  
A lo-fi on purpose · B honest mid-fi · C hi-fi · D sterile over-loud

**P3-Q06** Punchy dynamics or a brick-walled shout??  
→ `DYNAMIC_RANGE_LRA`  
A lots of air · B normal · C loud · D no quiet left

**P3-Q07** Mono-in-the-chest, wide stereo, or surrounding you??  
→ `SPATIAL_WIDTH`  
A mono / center · B stereo · C extra wide · D immersive / binaural

**P3-Q08** Surface of the sound.  
→ `TEXTURE_GRAIN.SURFACE`  
A glassy · B wooden / organic · C metallic · D liquid / wet

**P3-Q09** Any dirt they left in on purpose??  
→ `ARTIFACTS`  
A none · B vinyl crackle · C tape hiss · D hum / glitch / clip

**P3-Q10** Could you wear this mix like skin, or does it sit on top of you??  
→ intimacy of production  
A on the skin · B in the room · C across the street · D behind glass

**P3-Q11** Club system or earbuds at 2am??  
→ translation  
A designed for stacks · B both · C headphone secret · D nothing survives cheap speakers

**P3-Q12** Does the mix get harsher when it gets louder, or stay sweet??  
→ saturation under level  
A stays sweet · B blooms · C gets nasty · D already nasty

**P3-Q13** If you shut your eyes, is there a room around this or just plugins??  
→ space / reverb character  
A a real room · B a designed hall · C digital halo · D bone-dry

---

## Pillar 4 — LINGUISTIC · The Voice

Harvest: TIDAL writers, lyrics partners, language tags. Not artist Wikipedia.

**P4-Q01** Are they singing poetry, spitting fire, or telling you something they shouldn’t??  
→ `DELIVERY_STYLE` + `NARRATIVE_ARC`  
A poetry · B fire / bars · C raw confession · D no words, just voice as instrument

**P4-Q02** How filthy is the language, honest??  
→ `EXPLICIT_FILTER`  
A Clean · B Mild · C Explicit · D Severe

**P4-Q03** What is this actually about, in your gut??  
→ `TOPIC_CLUSTERS` (multi later; pick one now)  
A love / want · B money / flex · C pain / dark · D party / rebellion / other

**P4-Q04** Story with a beginning, or a mantra they keep carving??  
→ `NARRATIVE_ARC`  
A linear story · B scenes · C repetitive mantra · D abstract / no plot

**P4-Q05** How close is that mouth to your ear??  
→ `VOCAL_TEXTURE.POSITION`  
A intimate / almost touching · B in the room · C far / washed · D buried in the mix

**P4-Q06** Sung, rapped, screamed, spoken, or whispered like a secret??  
→ `DELIVERY_STYLE`  
A sung · B rapped / spoken · C scream / shout · D whisper / breath

**P4-Q07** Dry and naked, or processed until it’s a machine??  
→ `PROCESSING`  
A dry · B tasteful tune · C auto / vocode heavy · D chopped / screwed / destroyed

**P4-Q08** What language is the body of it??  
→ `PRIMARY_LANGUAGE`  
A English · B Spanish · C mix / code-switch · D other / I don’t know

**P4-Q09** Can you hear a city or a scene in the slang??  
→ `DIALECT_SLANG`  
A generic pop · B a real regional mouth · C scene slang (drill, phonk, raver…) · D made-up / alien

**P4-Q10** Would you let this play in front of someone’s mom??  
→ explicit + social  
A yes · B maybe skip one line · C no · D especially no

**P4-Q11** Are the words load-bearing, or could this be an instrumental and you wouldn’t miss them??  
→ semantic weight  
A the words *are* the song · B they help · C texture · D instrumental enough

**P4-Q12** Do they sound like they mean it, or like they’re performing meaning??  
→ delivery honesty  
A they mean it · B mixed · C costume · D can’t tell / no vocal

**P4-Q13** After one listen, could you quote a line that actually did something to you??  
→ hook of language  
A already can · B one phrase · C vibe only · D nothing stuck

---

## Pillar 5 — AFFECTIVE · The Heart

Harvest: none that is trustworthy. This pillar is almost all Tribe + scoring from audio energy. Thayer: valence −1…+1, arousal 0…1.

**P5-Q01** Where does this yank your chest first — euphoria, ache, rage, or a cold focus??  
→ `THAYER_COORDINATES` quadrant  
A euphoria (+val, high ar) · B melancholy (−val, low) · C rage (−val, high) · D focus / still (+val or 0, mid)

**P5-Q02** After the last bar, do you feel more alive or more emptied??  
→ `VALENCE`  
A lifted (+0.7) · B okay (+0.2) · C heavier (−0.4) · D wrecked (−0.85)

**P5-Q03** How hard does it run your nervous system??  
→ `AROUSAL`  
A almost still (0.15) · B awake (0.4) · C amped (0.7) · D redline (0.95)

**P5-Q04** Does it make you feel like you could take the room, or like you should hide in someone??  
→ `DOMINANCE`  
A empowering · B even · C vulnerable · D small / crushed

**P5-Q05** Same feeling the whole way, or mood swings??  
→ `MOOD_STABILITY`  
A locked · B gentle drift · C verse/chorus flip · D volatile

**P5-Q06** Could you cry, scream, or finally exhale to this on purpose??  
→ `CATHARSIS_POTENTIAL`  
A yes, that’s what it’s for · B if you’re already close · C not really · D it bottles you up

**P5-Q07** Does this smell like a year you already lived??  
→ `NOSTALGIA_TRIGGER`  
A immediately · B a little · C new / now · D outside of time

**P5-Q08** How fast does the tension climb??  
→ `BUILD_UP_VELOCITY`  
A it never builds · B slow burn · C standard rise · D vertical

**P5-Q09** How does it leave you when it’s over??  
→ `RESOLUTION_STATE`  
A triumphant · B soft landing · C melancholy · D unresolved / cliff

**P5-Q10** Would you send this to someone instead of a text you’re scared to write??  
→ intimacy / valence  
A that’s the move · B maybe · C too much · D wrong tool

**P5-Q11** Safe for a bad night, or will it make the bad night worse??  
→ care flag (not a schema leaf; store under tribe notes)  
A holds you · B depends · C risky · D do not, not tonight

**P5-Q12** One-word weather in the chest.  
→ mood label  
A bliss · B chaos · C ache · D steel

**P5-Q13** If this were a person, would you let them drive??  
→ dominance + trust  
A they can drive · B we switch · C I drive · D I get out

---

## Pillar 6 — CONTEXTUAL · The Scene

Harvest: Billboard era, 1001Tracklists, TIDAL album context. Tribe owns *where it belongs tonight*.

**P6-Q01** Where is the honest best place for this — late night drive, rain window, workout, or a sunny field??  
→ `MACRO_SETTING` + `MICRO_ACTIVITY`  
A night drive · B rain / room · C gym / work · D sun / festival

**P6-Q02** Big container.  
→ `MACRO_SETTING`  
A car · B bedroom · C gym / club · D outside / nature / office

**P6-Q03** What are your hands doing??  
→ `MICRO_ACTIVITY`  
A driving · B lifting / dancing · C deep work / still · D intimacy

**P6-Q04** Alone, two bodies, a kitchen full, or a crowd??  
→ `SOCIAL_SETTING`  
A solo · B couple · C small group · D mass

**P6-Q05** What is this *for*??  
→ `FUNCTIONAL_GOAL`  
A hype · B focus · C sleep / come-down · D seduction / filter

**P6-Q06** Clock time.  
→ `TIME_OF_DAY`  
A morning · B golden hour · C late night · D 3AM

**P6-Q07** Weather it summons.  
→ `WEATHER`  
A sun · B rain · C fog · D storm / snow

**P6-Q08** Temperature of the air around it.  
→ `TEMPERATURE`  
A cold / digital · B cool · C warm / analog · D hot

**P6-Q09** Would GYM_PEAK let this through, or kick it??  
→ context-engine hint (BPM/arousal still must match numbers)  
A gym peak yes · B gym warm-up · C night drive yes · D neither — private track

**P6-Q10** Headphones world or speakers-in-a-room world??  
→ playback context  
A sealed headphones · B car · C house speakers · D stacks / PA

**P6-Q11** Could you put this on around people you don’t know yet??  
→ social risk  
A yes, icebreaker · B after they get me · C only my people · D only me

**P6-Q12** Season, if it has one.  
→ era-adjacent context  
A summer · B autumn · C winter · D spring / any

**P6-Q13** If the night only had one song left, is this the closer, the opener, or the deleted scene??  
→ role  
A opener · B peak · C closer · D secret track

---

## Pillar 7 — PHOTOMETRIC · The Eye

Harvest: official art as a hint only. Hex must still be voted or derived from scoring. AuraOrb / lighting reads this pillar.

**P7-Q01** You have the laser desk. What color owns the room??  
→ `PRIMARY_HEX`  
A crimson / blood `#C41E3A` · B cobalt / night `#1B3B6F` · C gold / skin `#C9A227` · D void / UV `#5B2C6F`

**P7-Q02** Second color that answers it.  
→ `SECONDARY_HEX`  
A white slash `#F5F5F5` · B acid green `#39FF14` · C amber `#FF8C00` · D no second / monochrome

**P7-Q03** Warm body heat or cold steel??  
→ `PALETTE_TEMPERATURE`  
A warm · B neutral · C cool · D ice

**P7-Q04** Darkest the lights should ever sit.  
→ `BRIGHTNESS_FLOOR`  
A near black (0.05) · B dim club (0.2) · C room-on (0.45) · D never dim (0.7)

**P7-Q05** Brightest it deserves.  
→ `BRIGHTNESS_CEILING`  
A still moody (0.4) · B club peak (0.7) · C strobe-white (0.95) · D daylight (1.0)

**P7-Q06** Do the lights need to blink with the snare??  
→ `STROBE_TRIGGER`  
A no · B on the drop only · C often · D seizure-careful / I want it anyway

**P7-Q07** Cuts or melts??  
→ `FADE_RATE`  
A hard cut · B fast fade · C slow melt · D always breathing

**P7-Q08** How much haze??  
→ `FOG_DENSITY`  
A none (0) · B a veil (0.3) · C thick (0.7) · D can’t see the booth (0.95)

**P7-Q09** Beams — yes or they’ll look stupid??  
→ `LASER_COMPATIBILITY`  
A yes, this is a laser song · B some accents · C no · D video / not lights

**P7-Q10** Clean shapes or static and glitch??  
→ `VISUAL_NOISE`  
A clean / solid · B a little grain · C glitch · D full snow

**P7-Q11** Album art — trust it for color, or the sound is a different painting??  
→ art vs ear  
A art is right · B close · C ignore the art · D no art / don’t care

**P7-Q12** Eyes open or eyes shut??  
→ visual need  
A I need a show · B a little light · C darkness is the show · D either

**P7-Q13** If AuraOrb only got one instruction for the whole track.  
→ summary hex + fade  
A hold one color · B pulse the kick · C follow the lift · D chaos / flicker

---

## Pillar 8 — KINETIC · The Body

Harvest: BPM helps; MET is Tribe + scoring. This is the body vote.

**P8-Q01** How would you dance with a sexy person of interest to this??  
→ `MET_SCORE` + motor  
A slow grind / close (MET ~2.8) · B shuffle / footwork (~6.5) · C head-nod / sway (~1.8) · D mosh / peak (~8.5)

**P8-Q02** Where does your heart want to sit??  
→ `TARGET_HR_ZONE`  
A rest-adjacent · B easy (110s) · C work (130s) · D red

**P8-Q03** Does this calm your variability or squeeze it??  
→ `HRV_IMPACT`  
A flow / high HRV · B neutral · C a little stress · D clenched

**P8-Q04** Breathing — with it or against it??  
→ `BREATH_RATE`  
A long / slow · B walk-breath · C dance-breath · D I forget to breathe

**P8-Q05** Do your feet want to go forward??  
→ `MOTOR_RESPONSE.DRIVE`  
A stay · B stroll · C walk / run · D sprint

**P8-Q06** Hips.  
→ `SWAY`  
A locked · B a little · C yes · D they took the wheel

**P8-Q07** Neck.  
→ `HEAD_NOD`  
A none · B yes on the 2 and 4 · C constant · D full bang

**P8-Q08** Honest MET — sitting, standing, sweating, destroyed??  
→ `MET_SCORE`  
A ~1.5 sit · B ~3 sway · C ~6 dance · D ~9 fight the air

**P8-Q09** Could you hold someone and still move, or do you need space to not take them out??  
→ proximity  
A hold them · B close but bounce · C need a lane · D clear the pit

**P8-Q10** Hands: on a body, in the air, on the wheel, or fists??  
→ activity class  
A on a body · B in the air · C on the wheel / bar · D fists / impact

**P8-Q11** After five minutes, are you warmer or just prettier??  
→ expenditure check  
A just prettier · B warm · C sweat · D wrecked

**P8-Q12** Gym-legal??  
→ feeds context engine with Kinetic  
A heavy set yes · B cardio yes · C stretch / walk · D not for the gym

**P8-Q13** If they grabbed your hand mid-track, what happens??  
→ closer, same spirit as Q01  
A you pull them in · B you both bounce · C you keep nodding · D you lose them in the pit

---

## Pillar 9 — GENEALOGICAL · The Roots

Harvest: MusicBrainz + Discogs + TIDAL credits + Billboard. Tribe owns authenticity and tribe-id.

**P9-Q01** Does this stay true to its people, or is it dressed for the mall??  
→ `AUTHENTICITY_SCORE`  
A bone-true (0.9) · B mostly (0.7) · C crossover (0.4) · D costume (0.15)

**P9-Q02** What year does your body think this is from??  
→ `CULTURAL_ERA`  
A now · B 2010s · C 90s–00s · D older / timeless / future

**P9-Q03** Dated production, or could it walk into any decade??  
→ `TIMELESSNESS_SCORE`  
A stamped to a year (0.2) · B of-its-time (0.45) · C walks around (0.75) · D outside time (0.95)

**P9-Q04** Do you hear someone else’s record inside this??  
→ `SAMPLE_LINEAGE`  
A obvious sample · B interpolation I can name · C maybe · D feels original

**P9-Q05** If you can name the ghost, name the family.  
→ `INTERPOLATION` / lineage note  
A Amen / break world · B soul / disco ghost · C riff / melody steal · D I cannot name it

**P9-Q06** Genre tree, honest, not the iTunes bucket.  
→ `GENRE_TREE`  
A underground EDM / bass · B hip-hop / rap blood · C rock / metal / punk · D pop / other / hybrid

**P9-Q07** Who is this *for*??  
→ `SUBCULTURE_ID`  
A ravers · B heads / metal / punk · C hip-hop tribe · D pop / everybody / audiophile

**P9-Q08** Would the original scene claim this or side-eye it??  
→ `AUTHENTICITY_SCORE` check  
A claim it · B argue about it · C side-eye · D they would walk out

**P9-Q09** Is this about to be a meme, or is it too real to trend??  
→ `VIRAL_VELOCITY`  
A already spreading · B could · C slow burn · D will never be TikTok

**P9-Q10** Credits — did the names you know show up, or is this a ghost crew??  
→ credit awareness (TIDAL/MB)  
A I know these names · B one name · C nobody I know · D I don’t look

**P9-Q11** Vinyl in the blood, or born digital??  
→ pressing / medium feel  
A vinyl / analog lineage · B both · C digital native · D don’t care

**P9-Q12** Would you put this on a mix for your people without explaining yourself??  
→ tribe fit  
A instantly · B with a grin · C I’d warn them · D I’d hide it

**P9-Q13** One sentence you’d tell a kid in 20 years about where this came from.  
→ free-text Tribe note (not a leaf; stored as `tribe_origin_note`)  
A it came from the street / booth · B it came from a chart · C it came from a bedroom · D I still don’t know — leave unknown

---

## Geometry

| | Count |
| --- | --- |
| Pillars | 9 |
| Questions / pillar | 13 |
| Total cells | 117 |
| Choices / question | 4 + Skip |

13 is the same count as the other Aurphyx 13-section standards. A full pillar is a 13-beat. A full track is 9 × 13. Nobody owes a full track in one sitting. One honest Q01 is already more than a Like.

---

## Player copy (short)

Title tab: **VASP**  
Pillar page header: pillar name + archetype  
Footer: `Skip · Save · Submit to the Vibe Tribe`  
Empty field language: **unknown** — not “Local playback.”

If they only answer P8-Q01, write MET + dance class and leave the rest of Kinetic `null`. That is success.
