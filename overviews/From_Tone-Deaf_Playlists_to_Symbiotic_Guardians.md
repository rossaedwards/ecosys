
	Audio file
From_Tone-Deaf_Playlists_to_Symbiotic_Guardians.m4a
Transcript
00:00:00 Speaker 1
I had this moment the other night that I think just, it perfectly sums up the weird place we're at with technology.
00:00:06 Speaker 2
Oh yeah.
00:00:06 Speaker 1
Let's do that late night drive, you know, the one empty highway, maybe 2 in the morning, just a really introspective headspace.
00:00:13 Speaker 2
I know that drive well.
00:00:14 Speaker 1
And I'm just sort of staring out into the darkness, processing the week, and I'm trusting my streaming algorithm to, you know, soundtrack this very specific moment for me.
00:00:24 Speaker 2
The modern DJ, the ghost in the machine.
00:00:26 Speaker 1
Right, the invisible DJ.
00:00:27 Speaker 1
And honestly,
00:00:29 Speaker 1
It failed spectacularly.
00:00:32 Speaker 2
Uh-oh.
00:00:33 Speaker 2
What'd it do?
00:00:33 Speaker 1
I was listening to this really dark, atmospheric synth wave track, perfect vibe.
00:00:38 Speaker 1
And the second it ends, the algorithm throws on walking on sunshine.
00:00:43 Speaker 2
Oh no, that's emotional whiplash.
00:00:44 Speaker 1
It was so jarring.
00:00:46 Speaker 1
It didn't just break the flow.
00:00:47 Speaker 1
It felt
00:00:48 Speaker 1
I don't know, almost insulting.
00:00:49 Speaker 1
Like the machine knew nothing about me at all.
00:00:51 Speaker 2
That's the problem, isn't it?
00:00:52 Speaker 2
It's algorithmically correct, but emotionally tone deaf.
00:00:55 Speaker 1
That's the perfect phrase for it.
00:00:57 Speaker 2
Knows you like both of those songs statistically.
00:01:00 Speaker 2
It has the data, you've clicked on them before.
00:01:02 Speaker 2
It just has zero idea why you like them, or more importantly, when you need them.
00:01:06 Speaker 1
Exactly.
00:01:07 Speaker 1
It felt like I was in a library
00:01:09 Speaker 1
maybe tearing up a bit in the corner, and the librarian just starts throwing books at me based on my checkout history, completely oblivious to my state.
00:01:17 Speaker 2
You like The Great Gatsby, here's a Dr.
00:01:19 Speaker 2
Seuss book.
00:01:19 Speaker 1
Exactly.
00:01:21 Speaker 1
And that feeling, that frustration is actually the perfect launchpad for what we're unpacking today.
00:01:25 Speaker 2
It really is.
00:01:26 Speaker 1
We're looking at a stack of documents that frankly blew my mind.
00:01:30 Speaker 1
It starts by trying to solve that exact playlist problem, but then it takes this
00:01:35 Speaker 1
this massive left turn.
00:01:36 Speaker 2
A huge one.
00:01:37 Speaker 1
We start with fixing music metadata.
00:01:40 Speaker 1
And by the end of these papers, we're talking about redesigning how a blind person navigates a city using fiber optic railways.
00:01:47 Speaker 2
It's a wild ride.
00:01:48 Speaker 2
We're looking at the Vibe Audio Protocol, which they call VAP, then this SoulSync identity layer.
00:01:53 Speaker 2
And it all culminates in this massive concept called SAIL.
00:01:57 Speaker 1
And there was this tagline in the source material that really stuck with me.
00:02:00 Speaker 1
from user to soul.
00:02:02 Speaker 2
It sounds incredibly lofty, I know.
00:02:04 Speaker 2
But when you dig in, the engineering underneath it is surprisingly grounded.
00:02:09 Speaker 1
And I think we have to mention the catalyst here, right at the top.
00:02:12 Speaker 1
This is important.
00:02:13 Speaker 2
Absolutely.
00:02:13 Speaker 2
This wasn't some project cooked up in a lab at Spotify or Apple.
00:02:16 Speaker 1
Well, the collider.
00:02:18 Speaker 1
who goes by Arfix, states very clearly that this whole thing was born from a desire to create a symbiotic AI guardian for his best friend, Thomas, who is blind.
00:02:29 Speaker 2
And that just recontextualizes everything, doesn't it?
00:02:31 Speaker 1
Completely.
00:02:32 Speaker 1
Suddenly we're not just tagging MP3s for fun.
00:02:34 Speaker 1
We're on a mission to translate the very feeling of the world into data.
00:02:38 Speaker 2
Precisely.
00:02:39 Speaker 2
It's about bridging what they call the metadata gap.
00:02:42 Speaker 2
The gap between raw data and lived human experience.
00:02:45 Speaker 1
So let's start there.
00:02:46 Speaker 1
The metadata gap.
00:02:48 Speaker 1
I love that concept.
00:02:49 Speaker 1
We're all used to seeing that little text scroll across our card dashboard, right?
00:02:53 Speaker 1
Artist, title, album.
00:02:55 Speaker 2
The good old ID3 tag.
00:02:56 Speaker 2
It's the standard we've been using since, what, the mid-90s?
00:02:59 Speaker 1
Ancient history and tech terms.
00:03:01 Speaker 2
It's the Dewey Decimal System of Music.
00:03:03 Speaker 2
It's purely bibliographic.
00:03:05 Speaker 2
It tells you who made it and what it's called.
00:03:07 Speaker 1
But nothing else.
00:03:08 Speaker 2
Absolutely nothing about the experience contained in that file.
00:03:11 Speaker 2
It's like labeling a bottle of Carolina Reaper hot sauce, red liquid, 5 oz.
00:03:16 Speaker 1
Right, without the crucial detail
00:03:18 Speaker 1
that it's going to incinerate your entire digestive system.
00:03:21 Speaker 2
Exactly.
00:03:21 Speaker 2
The most important information is missing.
00:03:24 Speaker 1
It's blind to biology.
00:03:26 Speaker 1
It has no idea if a song is going to raise my heart rate or calm me down.
00:03:30 Speaker 1
It doesn't know if it's for a funeral or a rave.
00:03:32 Speaker 2
And that's the gap the Vibe Audio Protocol or VAP is designed to fill.
00:03:38 Speaker 2
The documents call it a shift to a holographic identity for every piece of audio.
00:03:42 Speaker 1
Holographic identity.
00:03:44 Speaker 1
I like that.
00:03:44 Speaker 1
So instead of a few lines of text, it's a full 3D picture of the song.
00:03:48 Speaker 2
That's the idea.
00:03:49 Speaker 2
It analyzes the audio file across 9 distinct pillars and it does it in under 200 milliseconds.
00:03:55 Speaker 1
Nine pillars.
00:03:56 Speaker 1
Okay, I really want to go through these one by one because as I was reading the Taxonomy Explorer document, I realized how limited my own
00:04:02 Speaker 1
vocabulary for music actually is.
00:04:04 Speaker 1
I'm usually just stuck on fast, slow, happy, sad.
00:04:08 Speaker 2
I think most of us are.
00:04:09 Speaker 2
But VAP doesn't treat music as art, not at first.
00:04:12 Speaker 2
It treats it as physics and psychology.
00:04:14 Speaker 2
It just breaks the sound wave down into its fundamental components.
00:04:18 Speaker 1
Okay, let's dive in.
00:04:19 Speaker 1
Pillar one, structural.
00:04:22 Speaker 1
The skeleton.
00:04:23 Speaker 1
This goes way beyond BPM, right?
00:04:24 Speaker 2
Oh, way beyond.
00:04:25 Speaker 2
BPM is just a number.
00:04:27 Speaker 2
It's the speed limit, that's all.
00:04:28 Speaker 1
Yeah, because 120 BPM could be a rigid military march, or it could be a groovy disco track.
00:04:34 Speaker 1
Totally different feel.
00:04:35 Speaker 2
Exactly.
00:04:36 Speaker 2
This pillar looks at what they call percussive DNA.
00:04:39 Speaker 1
Percussive DNA.
00:04:40 Speaker 1
Love that.
00:04:41 Speaker 2
Think about a kick drum.
00:04:42 Speaker 2
It's the heartbeat of most modern music.
00:04:45 Speaker 2
But not all heartbeats are the same.
00:04:47 Speaker 2
In a speed metal track, that kick is a sharp, high frequency click.
00:04:51 Speaker 2
It's all attack.
00:04:52 Speaker 2
It's designed to cut through a wall of distorted guitars.
00:04:55 Speaker 2
It's an assault.
00:04:56 Speaker 1
It's like a needle.
00:04:56 Speaker 1
It's pointy.
00:04:57 Speaker 2
A needle, that's a perfect word for it.
00:04:59 Speaker 2
The system tags that as having a sharp click transient profile.
00:05:03 Speaker 2
But then you take a trap or a hip hop track.
00:05:05 Speaker 1
The 808.
00:05:06 Speaker 2
The 808.
00:05:07 Speaker 2
That's a boom sub.
00:05:08 Speaker 2
It's a long rolling wave of low frequency pressure that physically moves the air and rattles your car.
00:05:13 Speaker 1
So VAP measures that.
00:05:16 Speaker 1
shape.
00:05:17 Speaker 1
the shape of the impact.
00:05:18 Speaker 2
Precisely.
00:05:18 Speaker 2
It asks, is this a puncture wound or is it a blunt force impact?
00:05:23 Speaker 2
The visceral effect is completely different and now the data reflects that.
00:05:26 Speaker 1
Wow, okay.
00:05:27 Speaker 1
What else is in the structural pillar?
00:05:29 Speaker 2
The other key metric is the syncopation index.
00:05:32 Speaker 2
This is all about the feel or the groove.
00:05:34 Speaker 1
This is the human swing versus machine lock thing I saw.
00:05:37 Speaker 2
Exactly that.
00:05:38 Speaker 2
Is every single beat hitting the millisecond grid with robotic precision, like a German techno track?
00:05:44 Speaker 1
Which creates a certain feeling, right?
00:05:46 Speaker 1
Hypnotic.
00:05:46 Speaker 1
driving, maybe a little cold.
00:05:48 Speaker 2
It creates focus, drive, a sense of artificiality.
00:05:52 Speaker 2
Or is it more like a J Dilla beat, where the snare is intentionally dragging just a little bit behind the beat?
00:05:58 Speaker 1
The drunken drummer feel.
00:05:59 Speaker 2
That in the pocket human feel.
00:06:01 Speaker 2
Or a jazz drummer who plays slightly ahead of the beat, creating pension.
00:06:04 Speaker 2
That tiny difference in timing triggers a completely different neurological response.
00:06:09 Speaker 2
One makes you focus, the other makes you relax and nod your head.
00:06:12 Speaker 1
And the machine can quantify that now, the swing.
00:06:14 Speaker 2
It can.
00:06:15 Speaker 2
It can tell you how human or how.
00:06:17 Speaker 1
Okay, so that's the skeleton, the math, the timing.
00:06:20 Speaker 1
It's pillar 2, tonal, the flesh.
00:06:23 Speaker 1
Right.
00:06:24 Speaker 2
If structure is the skeleton, tonal is the flesh on the bones.
00:06:27 Speaker 2
This is all about harmony.
00:06:29 Speaker 2
The notes themselves.
00:06:30 Speaker 1
The relationship between the notes.
00:06:31 Speaker 2
The system calculates something called dissonance density.
00:06:34 Speaker 2
Okay, so.
00:06:36 Speaker 1
Is that the nice it sounds?
00:06:37 Speaker 2
In a way.
00:06:38 Speaker 2
Is the music smooth, pleasant, mathematically correct, that's consonant, or is it full of clashing notes, tension, and unresolved chords?
00:06:47 Speaker 2
That's dissonant.
00:06:49 Speaker 1
Think like a pop ballad versus the soundtrack to a horror movie.
00:06:52 Speaker 2
Perfect example.
00:06:53 Speaker 2
One is designed to soothe you, the other to put you on edge.
00:06:57 Speaker 2
And the dissonance density score reflects that.
00:06:59 Speaker 1
I saw something else in this pillar that I know is a whole rabbit hole online.
00:07:03 Speaker 1
Tuning standards.
00:07:05 Speaker 1
440 Hertz versus 432.
00:07:07 Speaker 2
Yes, the cosmic tuning debate.
00:07:08 Speaker 1
So the standard for tuning an orchestra, a piano, everything is based on a note vibrating at 440 times per second, 440 Hertz.
00:07:17 Speaker 2
Correct.
00:07:18 Speaker 2
But there's a whole subculture, a big one, that believes tuning to 432 Hertz is more mathematically in tune with the universe, with nature, that it's a healing frequency.
00:07:28 Speaker 1
And what does VAP do with that?
00:07:30 Speaker 1
Does it take a side?
00:07:31 Speaker 2
No, and this is the clever part.
00:07:33 Speaker 2
It doesn't care if it actually heals your chakras or aligns your energy fields.
00:07:38 Speaker 2
It's agnostic.
00:07:39 Speaker 1
It just sees that the tuning is different.
00:07:40 Speaker 2
It just tags it.
00:07:41 Speaker 2
So if you're a user who says, I only want to listen to music with healing frequencies, the system can now find it for you.
00:07:48 Speaker 2
It serves the user's belief without validating the pseudoscience.
00:07:52 Speaker 1
That's smart.
00:07:52 Speaker 1
It's just another data point.
00:07:54 Speaker 1
Okay, Pillar 3, Timbrol.
00:07:57 Speaker 1
The skin.
00:07:58 Speaker 2
The texture.
00:07:59 Speaker 2
This is about the physics of the sound itself.
00:08:01 Speaker 2
It's why a guitar and a piano playing the exact same note sound completely different.
00:08:06 Speaker 1
So what's it measuring?
00:08:07 Speaker 2
One key metric is spectral physics.
00:08:10 Speaker 1
Sounds intimidating.
00:08:12 Speaker 2
It's just a fancy way of asking.
00:08:13 Speaker 2
What is the overall color of the sound?
00:08:15 Speaker 2
Is it dark or muddy with lots of congested frequencies in the low mids?
00:08:19 Speaker 1
Like an old recording maybe?
00:08:21 Speaker 2
Or a badly mixed metal song.
00:08:23 Speaker 2
Or is it bright and airy with lots of high frequency sheen and crispness?
00:08:27 Speaker 1
So it's like the EQ curve of the song.
00:08:29 Speaker 2
Essentially, yes.
00:08:30 Speaker 2
It also generates A fidelity score.
00:08:34 Speaker 2
Is this a pristine, high-resolution digital file, or does it have the crackle, the hiss, and the warmth of a vinyl record?
00:08:42 Speaker 1
Which is a huge vibe indicator.
00:08:44 Speaker 1
I mean, the entire genre of lo-fi beats to study to is built on that vinyl crackle.
00:08:50 Speaker 2
It's everything.
00:08:51 Speaker 2
That hiss isn't a flaw.
00:08:52 Speaker 2
It's an instrument.
00:08:53 Speaker 2
It's a signal to your brain that says nostalgia, relax, focus.
00:08:58 Speaker 2
If your metadata ignores that crackle, it misses the entire point of the genre.
00:09:02 Speaker 1
Right, the imperfection is the feature.
00:09:04 Speaker 1
Okay, this is making sense.
00:09:06 Speaker 1
Pillar 4 is linguistic, the voice.
00:09:09 Speaker 2
This is about the semantic message.
00:09:11 Speaker 2
The obvious one is, of course, checking for explicit lyrics.
00:09:13 Speaker 1
Sure, a parental advisory tag.
00:09:15 Speaker 2
But it goes deeper.
00:09:16 Speaker 2
It has an explicit severity tier.
00:09:18 Speaker 1
Meaning.
00:09:19 Speaker 2
Is it a casual curse word in a rap song or is it actual hate speech?
00:09:23 Speaker 2
The context and severity are very different, but the more interesting part for me is the vocal texture analysis.
00:09:28 Speaker 1
What does that capture?
00:09:29 Speaker 2
It looks at the recording itself.
00:09:31 Speaker 2
Is the singer whispering directly into your ear?
00:09:33 Speaker 2
That's the close mic proximity effect.
00:09:35 Speaker 2
It creates a feeling of intimacy.
00:09:36 Speaker 1
ASMR style.
00:09:38 Speaker 2
Exactly.
00:09:39 Speaker 2
Or are they shouting from a distance in a massive cave?
00:09:42 Speaker 2
That's high reverb.
00:09:44 Speaker 2
That creates a feeling of scale, of distance, of epicness.
00:09:47 Speaker 1
A whisper is personal, a shout is public.
00:09:50 Speaker 2
Precisely.
00:09:51 Speaker 2
And that distinction dramatically changes the listener's relationship to the song.
00:09:56 Speaker 2
Now, pillar 5 is the one I think directly solves your late night driving problem.
00:10:01 Speaker 1
The effective pillar, the heart.
00:10:02 Speaker 2
This is the emotional core.
00:10:04 Speaker 2
It uses the Fayer model, which is the gold standard for this kind of analysis.
00:10:08 Speaker 1
I vaguely remember this from a college psych class.
00:10:10 Speaker 1
It's a grid, right?
00:10:11 Speaker 2
It's a grid.
00:10:12 Speaker 2
It brilliantly stops using subjective words like happy or sad, and instead uses 2 vectors, valence
00:10:20 Speaker 2
and arousal.
00:10:21 Speaker 1
Okay, break that down.
00:10:21 Speaker 1
Valence is positive versus negative.
00:10:24 Speaker 2
You got it.
00:10:24 Speaker 2
High valence is positivity, pleasure, joy.
00:10:27 Speaker 2
Low valence is negativity, unpleasantness, displeasure.
00:10:30 Speaker 2
That's the vertical axis.
00:10:31 Speaker 1
And arousal is the horizontal one.
00:10:32 Speaker 2
Right, and that's simply energy.
00:10:34 Speaker 2
On the far left, you're asleep.
00:10:36 Speaker 2
On the far right, you're in a state of panic.
00:10:38 Speaker 1
So you can plot any emotion on this grid.
00:10:40 Speaker 2
Any emotion.
00:10:41 Speaker 2
So if you have high energy, high arousal, and high positivity, high valence,
00:10:46 Speaker 2
What do you get?
00:10:47 Speaker 1
That's euphoria.
00:10:49 Speaker 1
Joy, that's walking on sunshine, top right quadrant.
00:10:52 Speaker 2
Exactly.
00:10:52 Speaker 2
But now keep that high energy, keep the arousal maxed out, but drag the valence all the way down into the negative quadrant.
00:11:00 Speaker 1
So high energy, but a very bad feeling.
00:11:03 Speaker 1
That's anger.
00:11:05 Speaker 1
Aggression.
00:11:06 Speaker 2
That's your heavy metal.
00:11:06 Speaker 2
That's pure rage.
00:11:08 Speaker 2
Now what about your late night tribe?
00:11:09 Speaker 2
You were feeling introspective, maybe a little down.
00:11:11 Speaker 1
Yeah, definitely not high energy.
00:11:12 Speaker 1
So low arousal.
00:11:14 Speaker 1
And I wasn't feeling great.
00:11:15 Speaker 1
So negative valence.
00:11:16 Speaker 2
You were in the bottom left quadrant, melancholy, somber reflection, depression.
00:11:22 Speaker 1
And the algorithm threw a top right quadrant track at me.
00:11:25 Speaker 2
It teleported you across the emotional map.
00:11:27 Speaker 2
That's why it felt so violent, so jarring.
00:11:30 Speaker 2
A good system would have gently walked you from one quadrant to another, not thrown you across the chasm.
00:11:34 Speaker 1
That makes so much sense when you visualize it like that.
00:11:36 Speaker 1
It was an emotional mismatch.
00:11:38 Speaker 1
Okay, moving on.
00:11:39 Speaker 1
Pillar 6, contextual.
00:11:41 Speaker 1
The scene.
00:11:42 Speaker 2
This is where the AI starts to guess the intended use case for the music.
00:11:46 Speaker 1
The scenario engine.
00:11:47 Speaker 2
Right.
00:11:48 Speaker 2
It tags the track with macro settings.
00:11:50 Speaker 2
Is this gym music, night drive music, coffee shop music, bedroom music?
00:11:56 Speaker 1
And then there's the intent vector.
00:11:57 Speaker 2
Which is the functional goal.
00:11:59 Speaker 2
Is this music designed for focus?
00:12:01 Speaker 2
Is it for meditation?
00:12:02 Speaker 2
Or is it for seduction?
00:12:04 Speaker 1
Seduction as a metadata tag is fascinating.
00:12:08 Speaker 2
It is, isn't it?
00:12:09 Speaker 1
Implies the system has to understand the incredibly subtle cultural codes of romance and intimacy.
00:12:15 Speaker 2
It absolutely has to, because the last thing you want is a track with the seduction intent vector accidentally popping up
00:12:20 Speaker 2
playlist tagged for a toddler birthday party.
00:12:22 Speaker 1
Oh my God, no, definitely not.
00:12:24 Speaker 1
Okay, pillar 7 is photometric.
00:12:28 Speaker 1
The eye.
00:12:28 Speaker 1
This is where it gets visual, which is a weird concept for audio.
00:12:31 Speaker 2
It is, but it's about synesthesia, giving color to sound.
00:12:35 Speaker 2
VAP uses what it calls a chromatic map.
00:12:38 Speaker 1
How does it map a sound to a color?
00:12:39 Speaker 2
It's based on physics.
00:12:40 Speaker 2
Sound and light are both just waves, right?
00:12:43 Speaker 2
They're frequencies.
00:12:44 Speaker 2
Low frequency sound, like bass, has a long, slow waveform.
00:12:48 Speaker 2
Low frequency light, like the color red,
00:12:50 Speaker 2
also has a long wavelength.
00:12:52 Speaker 1
So it connects the two.
00:12:53 Speaker 1
Bass is red.
00:12:54 Speaker 2
Bass is red or even infrared.
00:12:56 Speaker 2
High frequency sound like cymbals or hi-hats has a short, fast waveform that maps to high frequency light like blue or ultraviolet.
00:13:05 Speaker 1
So a really bass heavy dubstep track would literally be tagged as red by the system.
00:13:09 Speaker 2
Deep red, yeah.
00:13:11 Speaker 2
And a really airy classical flute piece might be tagged as light blue.
00:13:15 Speaker 2
It also measures lumen dynamics.
00:13:17 Speaker 1
Mightness.
00:13:17 Speaker 2
Brightness, yeah.
00:13:19 Speaker 2
But also, how does that brightness change?
00:13:21 Speaker 2
Is it a steady glow or is it a strobe effect?
00:13:25 Speaker 2
This allows smart lights in your room to sync up to the music perfectly without any extra software.
00:13:30 Speaker 1
The music file itself tells the light bulbs what to do.
00:13:32 Speaker 2
Exactly.
00:13:33 Speaker 2
Okay, this is getting wild.
00:13:34 Speaker 2
Pillar 8.
00:13:36 Speaker 2
Kinetic, the body.
00:13:38 Speaker 1
How music makes you move.
00:13:40 Speaker 2
And I have to call out a specific metric in the documents here that made me do a double take.
00:13:44 Speaker 1
I have a feeling I know which one you're talking about.
00:13:45 Speaker 2
Stank phase drop squat.
00:13:47 Speaker 1
It's A legitimate technical tag in the VAP schema.
00:13:51 Speaker 2
It sounds like a ridiculous dance move, but it's actually capturing a very specific biological reaction, isn't it?
00:13:57 Speaker 2
is.
00:13:57 Speaker 2
It's part of the entrainment factor measurement.
00:13:59 Speaker 2
Entrainment is how a rhythm can take over your body's own rhythm.
00:14:04 Speaker 2
So the system asks,
00:14:05 Speaker 2
Does this audio create a body lock where you just sort of trance out and don't move much at all, like with some ambient music?
00:14:12 Speaker 2
Or does it have that specific kind of funky, syncopated groove that forces an involuntary motor response?
00:14:20 Speaker 2
The head nod, the shoulder shrug, the scrunched up face you make when the baseline is just,
00:14:26 Speaker 2
It's just nasty.
00:14:27 Speaker 1
The stank face.
00:14:28 Speaker 2
The stank face.
00:14:28 Speaker 2
We are codifying the nasty bass face into metadata.
00:14:31 Speaker 1
I love that.
00:14:32 Speaker 1
It's an admission that some music bypasses the conscious brain and just hits your motor cortex directly.
00:14:38 Speaker 2
It's vital data.
00:14:39 Speaker 2
If I'm trying to set a new personal record at the gym, I need that stank face drop squat energy.
00:14:43 Speaker 2
I don't want a body lock ambient drone.
00:14:46 Speaker 2
It also calculates a MET score for each track.
00:14:48 Speaker 1
MET as in metabolic equivalent?
00:14:51 Speaker 2
Exactly.
00:14:51 Speaker 2
It's a measure of energy expenditure.
00:14:54 Speaker 2
Is this music for sitting on the couch, Met score of 1, or is it for a full out sprint, Met score of 8 or 9?
00:15:01 Speaker 1
So it can build a workout playlist that actually matches the intensity of your intervals.
00:15:05 Speaker 2
That's the idea.
00:15:06 Speaker 1
Okay, finally, pillar 9, genealogical, the roots.
00:15:10 Speaker 2
This is about cultural lineage.
00:15:12 Speaker 2
Where did this sound come from?
00:15:13 Speaker 1
It tracks the obvious stuff, right?
00:15:15 Speaker 1
Like sampling history?
00:15:16 Speaker 2
Yes.
00:15:17 Speaker 2
This is the DNA of the track.
00:15:19 Speaker 2
If it samples A James Brown drum break, that's logged.
00:15:23 Speaker 2
But it also has a more subjective metric called tribe alignment.
00:15:27 Speaker 1
Tribe alignment.
00:15:27 Speaker 2
It uses a global voting system where communities self-identify.
00:15:32 Speaker 2
Think of it like subreddits.
00:15:33 Speaker 2
You have the goth tribe, the metalhead tribe, the ******** punk tribe.
00:15:37 Speaker 1
And they vote on authenticity.
00:15:38 Speaker 2
Exactly.
00:15:39 Speaker 2
So if a mainstream pop star suddenly releases a metal album, the system can look at the data.
00:15:44 Speaker 2
The structure in timber might say metal, but the metalhead tribe might vote it down, giving it a low tribe alignment score.
00:15:51 Speaker 1
It quantifies authenticity, or at least perceived authenticity.
00:15:55 Speaker 2
Which in music is often the same thing.
00:15:57 Speaker 1
Wow.
00:15:58 Speaker 1
Okay.
00:15:58 Speaker 1
The 9 pillars.
00:16:00 Speaker 1
That is incredibly comprehensive.
00:16:02 Speaker 2
It's a complete paradigm shift from artist title.
00:16:06 Speaker 1
And the documents apply this to a few case studies.
00:16:08 Speaker 1
And the contrast is just perfect.
00:16:11 Speaker 1
First up, Celine Dion, the power of love.
00:16:14 Speaker 2
A true epic.
00:16:15 Speaker 1
So VAP looks at it and sees 57 BPM.
00:16:19 Speaker 1
The structural tag is ballad slow burn.
00:16:22 Speaker 1
That makes sense.
00:16:23 Speaker 1
But the photometric pillar.
00:16:25 Speaker 1
The color was what got me.
00:16:26 Speaker 2
Hex code, #FFD700.
00:16:29 Speaker 2
Gold.
00:16:29 Speaker 1
Gold.
00:16:30 Speaker 1
Why gold?
00:16:31 Speaker 2
Because the system analyzes the frequencies, which are really lush, expansive, and centered in the mid-range with lots of reverb, and maps that to a pallet temperature it calls radiant warmth.
00:16:41 Speaker 1
And you know what?
00:16:42 Speaker 1
It feels gold.
00:16:42 Speaker 1
It sounds like a big radiant gold thing.
00:16:44 Speaker 2
Right, and the contextual tag is grand finale or hero moment.
00:16:47 Speaker 2
It knows this is the song that plays when the credits roll.
00:16:50 Speaker 1
Okay, now let's swing the pendulum.
00:16:51 Speaker 1
The other case study.
00:16:52 Speaker 1
Cannibal Corpse, Inhumane Harvest.
00:16:55 Speaker 2
The polar opposite.
00:16:56 Speaker 1
What does the system see there?
00:16:57 Speaker 2
It sees 140 BPM, but with the tag double time blast, recognizing the insane speed of the drumming.
00:17:03 Speaker 2
Effectively, it's valence negative.9, arousal is maxed out at 1.0.
00:17:08 Speaker 2
The tag is aggressive dominance.
00:17:10 Speaker 1
And the color.
00:17:10 Speaker 2
Hex code, hashtag 8000 Dark red.
00:17:14 Speaker 2
Blood cool is the official tag.
00:17:15 Speaker 2
It's literal synesthesia as a service.
00:17:17 Speaker 1
This is amazing.
00:17:18 Speaker 1
Okay, so that's the system for analyzing the world, or at least the world of audio.
00:17:23 Speaker 1
But how do I
00:17:25 Speaker 1
the listener plug into all this.
00:17:28 Speaker 2
Is where we get into the soul sync layer.
00:17:30 Speaker 2
And it starts with a concept called the soul shot.
00:17:32 Speaker 1
The soul shot.
00:17:33 Speaker 1
I have to admit, when I first read this part, my skepticism alarm went off a little bit.
00:17:37 Speaker 1
It started to sound...
00:17:39 Speaker 2
It definitely walks that line between hard math and what you could call mysticism.
00:17:45 Speaker 2
And I think that's a very deliberate choice.
00:17:47 Speaker 1
So it asks for my birth date, my exact time of birth, and my location.
00:17:51 Speaker 1
And from that, it generates my soul in.
00:17:53 Speaker 2
Right.
00:17:53 Speaker 2
But let's look at the mechanics of it.
00:17:55 Speaker 2
What's it actually doing?
00:17:56 Speaker 2
It takes the precise positions of the celestial bodies at that exact moment, Sun in Scorpio, Moon in Sagittarius, Aries rising, whatever it might be for you, and uses that unique astronomical configuration as a.
00:18:09 Speaker 1
A seed for what?
00:18:10 Speaker 2
A seed for a cryptographic hash.
00:18:13 Speaker 2
A SHA 256 hash to be specific.
00:18:15 Speaker 1
Okay, so it's not saying because you're a Scorpio, you like sad music.
00:18:18 Speaker 1
It's using the sky as a highly complex random number generator.
00:18:24 Speaker 2
That's a very technical way to put it, but yes.
00:18:26 Speaker 2
It's using the chaotic, unique state of the universe at your moment of birth to generate a genesis block for your digital identity.
00:18:33 Speaker 2
It's a unique starting point that can't be forged.
00:18:36 Speaker 1
But the creator, Orphix, they lean into the archetype.
00:18:39 Speaker 2
Heavily.
00:18:40 Speaker 2
The documents include this Orphix 13 month chaos and bliss calendar, which reintroduces the forgotten 13th zodiac sign, Ophiuchus.
00:18:49 Speaker 1
Ophiuchus the serpent bearer.
00:18:50 Speaker 2
Right, who they frame as the healer.
00:18:52 Speaker 2
The whole design aesthetic they show is this deep indigo maroon for Scorpio and metallic gold for alchemy.
00:18:58 Speaker 2
Very mystical.
00:19:00 Speaker 1
But the output of this mystical process is pure data.
00:19:03 Speaker 2
It's hard science.
00:19:04 Speaker 2
The soul shot process spits out your solid hash, but more importantly, it calculates a root frequency for you.
00:19:09 Speaker 1
A root frequency.
00:19:10 Speaker 1
For the example user in the docs, Ross, that frequency was 149.8 Hertz.
00:19:15 Speaker 2
Correct.
00:19:15 Speaker 1
What is the practical point of that?
00:19:17 Speaker 1
Why do I need to know that my personal frequency is 149.8 Hertz?
00:19:21 Speaker 1
Is this just for fun?
00:19:22 Speaker 2
No, this is where it gets really applied.
00:19:24 Speaker 2
In the SoulSync ecosystem, that frequency is your anchor.
00:19:28 Speaker 2
It's your biological calibration point.
00:19:31 Speaker 2
The core idea is
00:19:33 Speaker 2
Biometric entrainment.
00:19:34 Speaker 1
Entrainment.
00:19:34 Speaker 1
That's like when clocks in a room start ticking in sync, right?
00:19:38 Speaker 1
One rhythm influencing another.
00:19:39 Speaker 2
Precisely.
00:19:40 Speaker 2
SolSync is designed to constantly monitor your real-time biometrics.
00:19:44 Speaker 2
Through these nanomembrane sensors they describe, they're like ultra-thin, temporary tattoos you can wear on your wrist, no bulky straps or watches.
00:19:53 Speaker 2
They measure your heart rate variability, galvanic skin response.
00:19:56 Speaker 1
So it knows if I'm stressed or relaxed.
00:19:58 Speaker 2
It knows in real time.
00:19:59 Speaker 2
So let's say you're stressed out.
00:20:00 Speaker 2
Your heart rate is elevated and erratic.
00:20:03 Speaker 2
system knows your root frequency is 149.8 hertz.
00:20:06 Speaker 2
It can then dynamically shift the music you're hearing or even adjust the hue of the smart lights in your room to frequencies that are perfect harmonic intervals of 149.8 hertz.
00:20:16 Speaker 1
So it's not just playing a generic relaxing sounds playlist.
00:20:19 Speaker 2
No.
00:20:21 Speaker 2
It's creating an entire sensory environment that is mathematically tuned to your specific biology to physically pull your heart rate and nervous system back into coherence.
00:20:31 Speaker 2
It's A personalized biofeedback loop.
00:20:33 Speaker 1
That is next level.
00:20:34 Speaker 2
And it gets even wilder when you introduce other people into the equation.
00:20:37 Speaker 2
This is where they outline the five method.
00:20:39 Speaker 1
Oh, the social protocol.
00:20:40 Speaker 1
This part sounded a little aggressive, honestly.
00:20:43 Speaker 1
They call the first phase chaos mode.
00:20:45 Speaker 2
It's basically a social filter, a vibe check, weaponized.
00:20:50 Speaker 1
So explain it.
00:20:51 Speaker 1
You're in a car with someone new.
00:20:53 Speaker 1
Maybe it's a first date.
00:20:54 Speaker 1
What do you do?
00:20:55 Speaker 2
Phase one is the filter.
00:20:57 Speaker 2
You intentionally play music from the VAP's high arousal, low valence quadrant.
00:21:01 Speaker 1
So angry, aggressive music.
00:21:03 Speaker 2
Industrial, horrorcore.
00:21:04 Speaker 2
You play cannibal corpse and you watch their reaction.
00:21:07 Speaker 1
You're testing them.
00:21:08 Speaker 2
You're testing their emotional and energetic resilience.
00:21:10 Speaker 2
If they can't handle the dissonance, if they get annoyed or anxious or angry,
00:21:15 Speaker 2
The system logs their biometric response and marks them as having incompatible energy.
00:21:20 Speaker 1
That is unbelievably judgmental.
00:21:23 Speaker 1
Sorry, you can't ride in my car.
00:21:24 Speaker 1
You failed the chaos test.
00:21:25 Speaker 2
It is harsh.
00:21:26 Speaker 2
But the philosophy outlined in the document is if you can't handle the chaos, you can't handle the bliss.
00:21:32 Speaker 1
And phase two is the bliss mode.
00:21:33 Speaker 2
Phase 2 is the real in.
00:21:35 Speaker 2
If they pass the chaos test, you swing the pendulum.
00:21:38 Speaker 2
You shift to high valence music, soul, melodic bass, something warm and inviting.
00:21:44 Speaker 1
And the goal there.
00:21:45 Speaker 2
The stated goal is to achieve physiological synchrony.
00:21:49 Speaker 2
The system tries to sync your heart rates, your breathing patterns through the music to build a state of intimacy and connection.
00:21:55 Speaker 1
It's social engineering via a playlist.
00:21:57 Speaker 2
It is codified empathy or depending on your perspective, codified manipulation.
00:22:02 Speaker 1
It really feels like it could be both.
00:22:04 Speaker 1
But this is the moment in the documents where everything changes.
00:22:07 Speaker 1
We've been talking about optimizing playlists and judging first dates.
00:22:10 Speaker 1
And then the scope just, it explodes.
00:22:13 Speaker 1
We move from VAP to SAIL.
00:22:15 Speaker 2
The symbiotic accessibility intelligence layer.
00:22:18 Speaker 1
And this is where it all comes back to Thomas, the blind friend.
00:22:20 Speaker 2
Right.
00:22:21 Speaker 2
And I want to pause on this logical leap because it is genuinely brilliant.
00:22:25 Speaker 2
Okay.
00:22:25 Speaker 1
VAP, as we've established, is a system for describing the experience of sound.
00:22:30 Speaker 1
It tells you if a sound is warm or sharp or distant or
00:22:33 Speaker 1
threatening.
00:22:34 Speaker 2
Right, it's an experiential language.
00:22:36 Speaker 1
The realization was, can we use that exact same experiential language to describe physical reality for someone who can't see it?
00:22:44 Speaker 1
Can we give a physical room a vibe score?
00:22:47 Speaker 2
So mapping those same 9 pillars, but onto the real world instead of an audio file.
00:22:51 Speaker 1
Exactly.
00:22:52 Speaker 1
Let's run through it.
00:22:53 Speaker 1
Take the structural pillar.
00:22:54 Speaker 1
In music, it's the beat, the rhythm, the grid.
00:22:58 Speaker 1
In sale, structural becomes spatial geometry.
00:23:01 Speaker 1
It's the architecture of the room.
00:23:03 Speaker 1
Where are the walls?
00:23:04 Speaker 1
How far is that chair?
00:23:05 Speaker 1
He tells Thomas, chair, 1.2 meters ahead at your 10 o'clock.
00:23:09 Speaker 2
Simple enough.
00:23:10 Speaker 2
What about tonal?
00:23:11 Speaker 2
In music, that's harmony and distance.
00:23:13 Speaker 1
In sale,
00:23:14 Speaker 1
Tonal becomes the emotional tone of the environment based on visual cues.
00:23:18 Speaker 1
Is the lighting in this kitchen warm and calm?
00:23:20 Speaker 1
Or is the flickering fluorescent light in this subway station cold and tense?
00:23:25 Speaker 1
Wow.
00:23:26 Speaker 1
That's a huge piece of information to be missing.
00:23:29 Speaker 1
A blind person can use a cane to find obstacles, but they can't see the tension on people's faces in a room.
00:23:35 Speaker 2
And that's where the effective pillar gets mapped.
00:23:38 Speaker 2
The system uses cameras to analyze faces and body language.
00:23:41 Speaker 1
Like the Thayer model for the real world.
00:23:43 Speaker 2
Exactly.
00:23:44 Speaker 2
So it can whisper to Thomas, your friend just smiled widely.
00:23:48 Speaker 2
Or on the other end of the spectrum, caution.
00:23:51 Speaker 2
Tension detected in the room.
00:23:53 Speaker 2
Someone is raising their voice at your 3 o'clock.
00:23:55 Speaker 1
It's giving him the social and emotional context that he would otherwise miss.
00:23:59 Speaker 2
It's a window.
00:24:00 Speaker 2
And they use the contextual pillar to solve a huge problem in current assistive tech.
00:24:05 Speaker 1
Which is.
00:24:06 Speaker 2
Information overload.
00:24:07 Speaker 2
Most computer vision systems for the blind just list objects.
00:24:11 Speaker 2
They'll say, chair, table, floor, cat, window, lamp.
00:24:15 Speaker 2
It's just noise.
00:24:16 Speaker 2
It's overwhelming.
00:24:16 Speaker 1
You can't process that much data.
00:24:18 Speaker 1
You don't know what's important.
00:24:19 Speaker 2
Right.
00:24:19 Speaker 2
So SAIL uses something called reverse eye tracking or saliency models.
00:24:23 Speaker 1
Okay, what does that mean?
00:24:25 Speaker 2
These are AI models trained to predict what a sighted human's eye would
00:24:29 Speaker 2
naturally look at first in any given scene.
00:24:31 Speaker 1
Oh, cover.
00:24:32 Speaker 2
If a dog is running towards you, don't look at the texture of the pavement, you look at the dog.
00:24:37 Speaker 2
So the system prioritizes what's salient.
00:24:40 Speaker 2
It doesn't list everything.
00:24:41 Speaker 2
It says, warning, fast moving dog approaching on your right.
00:24:45 Speaker 2
It filters reality the way a sighted brain does.
00:24:48 Speaker 1
And this information is all delivered through an AI companion named Audrey.
00:24:53 Speaker 2
Audrey, the symbiotic AI guardian.
00:24:56 Speaker 2
She's the voice that fuses all these inputs together.
00:24:59 Speaker 2
She whispers these cues to Thomas via bone conduction audio and a pair of glasses so his ears are still free to hear the world.
00:25:06 Speaker 1
But it's not just audio, right?
00:25:07 Speaker 1
Yeah.
00:25:08 Speaker 1
She also uses haptics.
00:25:09 Speaker 2
Yes.
00:25:10 Speaker 2
And this is where the photometric pillar gets translated.
00:25:12 Speaker 2
This might be my favorite part of the whole system.
00:25:14 Speaker 1
Go on.
00:25:15 Speaker 2
How do you explain the concept of brightness to someone who has never seen light?
00:25:18 Speaker 2
It's an impossible task.
00:25:19 Speaker 2
Right.
00:25:20 Speaker 2
So the system doesn't try to explain it.
00:25:22 Speaker 2
translates it.
00:25:23 Speaker 2
maps light intensity picked up by cameras.
00:25:26 Speaker 2
to haptic vibration on his watch or on that nano membrane patch.
00:25:30 Speaker 1
So, if he walks into a sunbeam.
00:25:33 Speaker 2
He feels a warm, steady vibration increase on his skin.
00:25:37 Speaker 2
When he walks into a shadow, it fades away.
00:25:40 Speaker 2
If a light is strobing, he feels a distinct pulse.
00:25:43 Speaker 1
My God.
00:25:44 Speaker 2
He can feel the light.
00:25:46 Speaker 2
He gets a sense of the visual energy of a room without ever seeing it.
00:25:50 Speaker 1
That is.
00:25:51 Speaker 1
It's genuinely beautiful.
00:25:52 Speaker 1
It's taking technology that we usually associate with surveillance and isolation and using it to connect someone more deeply to the world around them.
00:26:02 Speaker 2
But the documents don't stop there.
00:26:03 Speaker 2
They don't just stop at the individual.
00:26:05 Speaker 2
They scale it up to the level of city infrastructure.
00:26:08 Speaker 1
The light reel.
00:26:09 Speaker 2
The photonic circuit railway.
00:26:10 Speaker 2
This is where it goes full sci-fi.
00:26:12 Speaker 1
Sounds like a teleportation device from a movie.
00:26:14 Speaker 2
It's actually based on real cutting edge technology called distributed acoustic sensing or DAS.
00:26:20 Speaker 1
Okay, you're gonna have to explain that one.
00:26:22 Speaker 2
Think about all the thousands of miles of fiber optic cables that already run alongside our train tracks and highways.
00:26:28 Speaker 1
For internet signaling, that kind of thing.
00:26:30 Speaker 2
Exactly.
00:26:31 Speaker 2
Now,
00:26:31 Speaker 2
If you shoot a pulsed laser down one of those cables, the light travels along the glass.
00:26:38 Speaker 2
Tiny microscopic imperfections in the glass, which are naturally there, reflect A minuscule amount of that light back to the source.
00:26:46 Speaker 1
Okay, a little bit of backscatter.
00:26:47 Speaker 2
Right.
00:26:48 Speaker 2
Now what happens if a train drives over the track next to that cable?
00:26:52 Speaker 2
Or if a person walks near it?
00:26:54 Speaker 2
creates a vibration.
00:26:56 Speaker 1
A tiny one.
00:26:56 Speaker 2
A tiny one.
00:26:57 Speaker 2
But that vibration stretches or compresses the glass fiber by nanometers.
00:27:02 Speaker 2
And that change in the fiber alters the reflection of the laser light.
00:27:06 Speaker 1
Wait, so you're saying the cable itself becomes a microphone?
00:27:09 Speaker 2
Not just a microphone, a continuous thousands of miles long sensor.
00:27:13 Speaker 2
It can feel the location, speed, and weight of a train, yes.
00:27:17 Speaker 2
But it can also feel the foot
00:27:18 Speaker 2
steps of a person walking on the tracks a mile away.
00:27:21 Speaker 2
You can feel a rock slide starting.
00:27:23 Speaker 2
You can feel a tiny change in temperature that might indicate a fire.
00:27:26 Speaker 1
The entire railway becomes a nervous system.
00:27:28 Speaker 2
A living nervous system.
00:27:29 Speaker 2
And this connects back to Thomas through SQUAL.
00:27:32 Speaker 1
Another acronym.
00:27:33 Speaker 2
The SoulSync Quantum Universal Accessibility Interface and Layer.
00:27:36 Speaker 2
It's the final piece.
00:27:37 Speaker 1
So how does the smart railway help Thomas?
00:27:40 Speaker 2
Imagine Thomas is riding on one of these autonomous light reel pods.
00:27:44 Speaker 2
The track
00:27:45 Speaker 2
itself is scanning the entire station ahead of him at the speed of light, using these autonic integrated circuits for processing.
00:27:53 Speaker 2
It knows with millimeter precision where the platform edge is, where a puddle of water is, where a piece of luggage has been left behind, where the crowd is thickest.
00:28:02 Speaker 1
And it beams that information directly to Audrey.
00:28:05 Speaker 2
Instantly.
00:28:06 Speaker 2
There's 0 latency because it's photonic.
00:28:08 Speaker 2
It's all happening at light speed.
00:28:09 Speaker 2
So Audrey whispers to Thomas, approaching downtown station, crowd density is high on the left side of the platform.
00:28:15 Speaker 2
The elevator is functional on your right.
00:28:17 Speaker 2
Tactile paving begins in 8 meters.
00:28:19 Speaker 1
It gives him a preview of the world he's about to enter.
00:28:22 Speaker 2
A perfect high resolution preview.
00:28:24 Speaker 2
They call it a living urban memory.
00:28:26 Speaker 1
The ORFS meshwork.
00:28:28 Speaker 2
Right.
00:28:29 Speaker 2
These decentralized nodes that cache these sale descriptions.
00:28:32 Speaker 2
So if a construction crew digs a hole in the sidewalk outside the station, the DAS system feels the vibration of the jackhammer, the map is updated in real time, and Audrey warns Thomas about it before he even leaves the train platform.
00:28:45 Speaker 1
It's a complete reimagining of what infrastructure is for.
00:28:48 Speaker 1
We build roads for cars.
00:28:50 Speaker 1
We build buildings for able-bodied people.
00:28:52 Speaker 1
This is designing an entire system for
00:28:56 Speaker 1
for the soul.
00:28:56 Speaker 2
For most vulnerable, for the human experience.
00:28:59 Speaker 2
It even mentions using flux sensors, fleets of vehicles with cameras to generate Rd.
00:29:04 Speaker 2
remodeling plans for the entire USA, using AI to identify and prioritize the most critical accessibility upgrades nationwide.
00:29:12 Speaker 1
It really does all tie back together, doesn't it?
00:29:14 Speaker 1
From classifying the kick drum in a death metal song.
00:29:17 Speaker 2
All the way to guiding a blind man through a photonic railway station.
00:29:20 Speaker 1
And the unifying thread is that solity, that 149.8 Hertz root frequency.
00:29:25 Speaker 1
song on your playlist, to the lights in your living room, to the train you take to work.
00:29:29 Speaker 2
It's the one account, one soul philosophy and practice.
00:29:32 Speaker 2
You know, we spend.
00:29:32 Speaker 1
So much of our time now talking about privacy, about the fear of surveillance.
00:29:37 Speaker 2
And justifiably so.
00:29:38 Speaker 2
We live in an age of surveillance capitalism.
00:29:40 Speaker 2
Right.
00:29:41 Speaker 1
I don't want the algorithm tracking me.
00:29:42 Speaker 1
I don't want the government watching me.
00:29:45 Speaker 1
But then you look at a system like this.
00:29:47 Speaker 1
Imagine a world where the environment is watching you.
00:29:52 Speaker 1
But it's not watching you to sell you a pair of shoes.
00:29:55 Speaker 2
It's watching you to see if you're stressed.
00:29:56 Speaker 1
It's watching to see if you're about to trip over an unseen curb.
00:30:00 Speaker 1
It knows your route frequency, and it subtly adjusts the ambient lighting to calm your nervous system down after a hard day.
00:30:07 Speaker 2
It completely reframes the question.
00:30:10 Speaker 1
It really does.
00:30:10 Speaker 1
The question stops being, am I being monitored?
00:30:14 Speaker 1
And it becomes something else entirely.
00:30:15 Speaker 2
What's that?
00:30:16 Speaker 1
In a world like that, would you feel monitored or would you finally feel
00:30:22 Speaker 1
understood.
00:30:23 Speaker 2
That is the question.
00:30:24 Speaker 1
Something to think about the next time your playlist completely betrays your mood.
00:30:28 Speaker 1
Thanks for diving with us on this.
00:30:30 Speaker 2
One account, one soul.
