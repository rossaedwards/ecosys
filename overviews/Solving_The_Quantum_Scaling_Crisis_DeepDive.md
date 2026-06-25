## Audio file 

Solving_The_Quantum_Scaling_Crisis_With_Fractals.m4a 

## Transcript 

00:00:00 Speaker 1 

Welcome back to the Deep Dive. Today, we are looking at a set of documents that, quite frankly, they read like a transmission from 20 years in the future. 

## 00:00:09 Speaker 2 

Yeah, they really do. It's the absolute bleeding edge. 

## 00:00:13 Speaker 2 

of quantum computing. 

00:00:14 Speaker 1 

Right. But, and this is a big but for you listening, we aren't talking about the usual headlines. 

00:00:20 Speaker 2 

No, we are definitely not. 

## 00:00:21 Speaker 1 

We aren't talking about IBM just, you know, slapping more quibits onto a chip or Google cooling a refrigerator down to absolute zero for the 100th time. 

## 00:00:29 Speaker 2 

Exactly. This is something fundamentally different. It's not about doing the same thing. 

00:00:33 Speaker 2 

but bigger. It is about rethinking the entire board game from scratch. 

## 00:00:37 Speaker 1 

And the board game in this case is a proposal called Arphyx, A-U-R-P-H-Y-X. 

00:00:42 Speaker 2 

Arphyx, right. 

## 00:00:43 Speaker 1 

And if this stack of papers we're analyzing today is accurate, I mean, this isn't just some minor engineering tweak. 

00:00:49 Speaker 2 

Not at all. 

## 00:00:50 Speaker 1 

It is a proposal to completely tear down the way we think about building these machines because they are arguing that the problem we're facing, the reason we don't have quantum supercomputers on our desks isn't the physics. 

## 00:01:02 Speaker 1 

It is the geometry. 

## 00:01:03 Speaker 2 

Geometry is destiny. That's really the core thesis here. And it is a incredibly dense, fascinating proposal. But to really grasp why this matters for you, the listener, we have to start with the elephant in the room. 

## 00:01:17 Speaker 1 

The scaling crisis. 

00:01:19 Speaker 2 

The scaling crisis, yes. 

00:01:19 Speaker 1 

So let's set the scene there. 

00:01:22 Speaker 1 

Because if you read the tech news lately, you would think we are almost there, right? 

00:01:25 Speaker 2 

Oh, absolutely. The PR is fantastic. 

00:01:27 Speaker 1 

I see the photos of those beautiful gold chandeliers, the cryostats. 

## 00:01:32 Speaker 2 

Steampunk looking machines, yeah. 

## 00:01:34 Speaker 1 

Exactly. And we hear about the thousand quipe chips. I mean, IBM's Condor, Google's Willow. It feels like the quantum revolution is happening right now in real time. 

## 00:01:44 Speaker 2 

It feels like it. But the documents we are analyzing today 

## 00:01:48 Speaker 2 

paint a much, much darker picture of the current state of affairs. 

## 00:01:50 Speaker 1 

Darker how? 

00:01:51 Speaker 2 

Well, it's the difference between having a prototype and having a product. 

## 00:01:54 Speaker 3 

Okay. 

## 00:01:55 Speaker 2 

We have the prototypes right now. We have machines that can do very specific, very small things. 

## 00:01:59 Speaker 2 

But the scaling crisis describes this massive wall that the entire industry is currently just, they are smashing their heads against it. 

## 00:02:07 Speaker 1 

And it all comes down to 1 dirty word in the quantum world, doesn't it? 

## 00:02:11 Speaker 2 

Noise. 

00:02:12 Speaker 1 

Noise. 

00:02:13 Speaker 2 

Yes. 

## 00:02:13 Speaker 1 

So explain that for us really quickly. When I say noise in a classical computer, I think of a fan whirring or maybe static on a screen. 

## 00:02:21 Speaker 2 

Right. Or 

00:02:22 Speaker 2 

Audio interference. 

00:02:23 Speaker 1 

Right. What exactly is noise in a quantum system? 

## 00:02:26 Speaker 2 

In a quantum system, noise is literally anything from the outside world. 

## 00:02:30 Speaker 1 

Anything. 

00:02:31 Speaker 2 

Anything. A stray photon of light, a vibration from a delivery truck driving by outside the lab, a tiny microscopic fluctuation in temperature, even the magnetic field of the Earth itself. 

00:02:44 Speaker 1 

Because the quibbits are that sensitive. 

00:02:46 Speaker 2 

They are incredibly fragile. 

## 00:02:48 Speaker 2 

Quantum states, the actual physical things doing the math, rely on these delicate properties called superposition and entanglement. 

## 00:02:55 Speaker 1 

Right, being in multiple states at once and being linked together. 

## 00:02:58 Speaker 2 

Exactly. And if they feel that noise, if the outside world interacts with them, even a tiny bit, they collapse. They just lose the information. 

00:03:05 Speaker 1 

So they make errors. 

## 00:03:06 Speaker 2 

Constantly. They make errors constantly. 

## 00:03:08 Speaker 1 

And to fix those errors, the current industry standard is something called surface codes. 

## 00:03:12 Speaker 2 

Surface codes, yes. 

## 00:03:13 Speaker 1 

Which, as I understand it from the source material, is basically the ultimate brute force method. 

## 00:03:19 Speaker 2 

It is entirely brute force to get one single logical quibit, 

## 00:03:23 Speaker 1 

meaning one bit of information that you can actually trust to do math. 

00:03:28 Speaker 2 

Exactly. One perfect, reliable quibit. To get that, you need to surround it with a whole choir of physical quibits. 

## 00:03:35 Speaker 2 

whose only job, their entire existence, is just to spot errors in the main quibit and correct them. 

## 00:03:41 Speaker 1 

And we are looking at the numbers in the proposal right here. The ratio is staggering. 

00:03:46 Speaker 2 

It is. It's somewhere between 1000 to 1 up to 10,000 to 1. 

00:03:49 Speaker 1 

10,000 to 1. 

00:03:50 Speaker 2 

In some architectures, like trapped ions or standard superconducting circuits, yes. 

## 00:03:54 Speaker 2 

depending on the base error rate. 

## 00:03:56 Speaker 1 

Just to pause on that, because I want the listener to really internalize this. If I want one useful bit of data, I might need 10,000 other physical particles just strictly for error correction. 

## 00:04:06 Speaker 2 

That is a reality of hardware right now. 

## 00:04:08 Speaker 1 

That feels like needing a support staff of 10,000 people just to get one CEO to sign a single piece of paper. 

## 00:04:14 Speaker 2 

That is a brilliant analogy. And here is why that is such a massive problem. 

00:04:18 Speaker 1 

The scaling. 

## 00:04:19 Speaker 2 

Right. To do anything actually useful for humanity, like simulating a 

## 00:04:24 Speaker 2 

a complex new drug molecule or cracking RSA encryption or optimizing global supply chains, we need thousands of logical quibits. 

## 00:04:33 Speaker 1 

Thousands of the perfect ones? 

## 00:04:34 Speaker 2 

Yes. So if you apply that 10,000 to 1 multiplier, you suddenly need a physical processor with millions, maybe 10s of millions of physical quibits on it. 

## 00:04:44 Speaker 1 

Which brings us to the wall. 

## 00:04:45 Speaker 2 

That is the scaling wall. Imagine the wiring for 10 million quibits. 

## 00:04:49 Speaker 1 

It's impossible. 

## 00:04:50 Speaker 2 

Imagine the cooling power needed to keep 10 million individual circuits 

## 00:04:54 Speaker 2 

at near absolute zero. It is a fundamental manufacturing nightmare. We can build 1000 quivits today. We cannot, with current Euclidean technology, build 10 million without the machine becoming the size of a football stadium and just melting itself down. 

## 00:05:07 Speaker 1 

And this is exactly where the Orphix proposal enters the chat. 

## 00:05:10 Speaker 2 

Yes, this is their grand entrance. 

## 00:05:12 Speaker 1 

We have this document here. The principal investigator is named Ross A. Edwards, and they are asking for funding. 

## 00:05:18 Speaker 2 

A very specific amount, too. 

## 00:05:20 Speaker 1 

Right. They want $1.25 million over an 18-month timeline to test a completely new architecture. 

## 00:05:29 Speaker 2 

Which is practically pocket change in the quantum hardware space. 

## 00:05:32 Speaker 1 

It really is. And their pitch is basically, stop trying to build a bigger grid, just change the shape of the computer. 

## 00:05:39 Speaker 2 

That is the hook that makes this so fascinating. They are 

## 00:05:42 Speaker 2 

a convergence of three very distinct, very complex fields of science. 

## 00:05:48 Speaker 1 

Let's lay those out for the listener. 

## 00:05:50 Speaker 2 

Okay, so we have fractal geometry, topological physics, and 

## 00:05:53 Speaker 2 

in photonics. 

## 00:05:54 Speaker 1 

Which sounds like an absolute buzzword salad when you say it all together. 

## 00:05:58 Speaker 2 

No, it really does. 

## 00:05:59 Speaker 1 

But going through the deep dive of this paper, it's actually a very elegant interlocked solution. They want to use fractals, specifically this shape called the Sierpieski gasket, to create geometric shortcuts in the computer's architecture. 

## 00:06:12 Speaker 2 

That's pillar one. 

00:06:13 Speaker 1 

Right. Then they want to use this theoretical particle called a neglectin to handle the actual logic without causing errors. 

00:06:20 Speaker 2 

Pillar 2. 

00:06:21 Speaker 1 

And finally, they want to build the whole physical thing out of light 

00:06:24 Speaker 1 

trapped in glass. 

00:06:24 Speaker 2 

Pillar 3, photonics. 

00:06:27 Speaker 1 

There is so much to unpack there. I mean, neglectins might be my favorite word of the day. 

00:06:31 Speaker 2 

It is a great term. 

00:06:32 Speaker 1 

It sounds like a forgotten character from a sad nursery rhyme. But let's take this step by step. We have the three pillars of this proposal. Let's start with pillar one, the fractal advantage. 

00:06:43 Speaker 2 

So we go back to that phrase. 

00:06:45 Speaker 2 

Geometry is destiny. 

00:06:47 Speaker 1 

The proposal argues that our current quantum chips are Euclidean. 

00:06:51 Speaker 2 

Euclidean, yes. Standard geometry. 

00:06:53 Speaker 1 

For those of us listening who haven't opened a geometry textbook since high school, what does Euclidean mean in the context of a computer chip? 

## 00:07:01 Speaker 2 

It means they are grids. Think of a checkerboard. 

## 00:07:04 Speaker 1 

Okay, like a literal physical grid of squares. 

## 00:07:07 Speaker 2 

Right, or a sheet of graph paper. The physical quibbits are arranged in rigid rows and columns. 

## 00:07:12 Speaker 1 

So if I have a quibbit at the bottom left of the chip and it needs to talk to a quibbit at the top right to do some math. 

## 00:07:19 Speaker 2 

To form an entanglement operation, right? 

## 00:07:20 Speaker 1 

That information has to hop step by step through every single neighbor in between them. 

## 00:07:25 Speaker 2 

It's like a local commute. 

## 00:07:26 Speaker 1 

You have to stop at every single traffic light and intersection in town. 

## 00:07:31 Speaker 2 

Exactly. And in quantum computing, that commute is deadly. 

## 00:07:34 Speaker 1 

because of the noise we talked about? 

## 00:07:36 Speaker 2 

Yes. Every hop introduces a new chance for a tiny error. Every hop takes a fraction of a nanosecond, which adds up. 

00:07:44 Speaker 1 

So the signal degrades. 

00:07:45 Speaker 2 

Maintaining entanglement, which is that spooky, delicate connection between distant particles across a grid, is incredibly expensive in terms of energy and what we call coherence. 

## 00:07:56 Speaker 1 

Coherence meaning the lifespan of the quantum state. 

## 00:07:59 Speaker 2 

Right. You basically lose the signal before it ever gets to its destination across the chip. 

00:08:03 Speaker 1 

So our fixed 

00:08:05 Speaker 1 

we just ditch the grid entirely, move to a fractal. 

## 00:08:07 Speaker 2 

Yes. 

## 00:08:07 Speaker 1 

Specifically, the Sierpieski gasket. Now, describe the shape for the listeners who might not be visualizing it or who haven't Googled it yet. 

## 00:08:16 Speaker 2 

Okay, so imagine a solid, flat, equilateral triangle on a piece of paper. 

## 00:08:20 Speaker 1 

Got it. 

## 00:08:21 Speaker 2 

Now, take a hole puncher that is also shaped like an upside-down triangle and punch out the exact middle of your big triangle. 

## 00:08:29 Speaker 1 

Okay, so I'm left with three smaller solid triangles that are touching at their corners. 

00:08:34 Speaker 2 

Exactly. Like the Triforce from the Legend of Zelda video games. 

00:08:37 Speaker 1 

Oh, perfect analogy. The Triforce. 

00:08:40 Speaker 2 

Now do it again. Take each of those three smaller solid triangles and punch the middle out of them. 

00:08:45 Speaker 1 

So now I have nine tiny triangles. 

## 00:08:48 Speaker 2 

Right. And you do it again and again, infinitely. 

## 00:08:51 Speaker 1 

You just keep punching out the middle of whatever is left. 

## 00:08:53 Speaker 2 

Yes. That resulting shape is the Sierpieski gasket. 

## 00:08:56 Speaker 1 

It ends up looking like a triangle made of triangles. 

## 00:08:59 Speaker 1 

triangles, but with microscopic holes everywhere, like geometric Swiss cheese. 

## 00:09:03 Speaker 2 

Swiss cheese is a good way to picture it. And because of all those holes, it has a very strange mathematical property. It has a Hausdorff dimension of roughly 1.585. 

## 00:09:13 Speaker 1 

Okay, wait, 1.585 dimensions. I think most people intuitively understand 1D a straight line and 2D a flat, flat square on a page. 

## 00:09:23 Speaker 1 

How can a physical shape be 1.585 dimensions? 

## 00:09:26 Speaker 2 

It's A mathematical measure of how rough or complex the shape is as you zoom in. It covers strictly more space than a 1D line because it spreads out. But it's not a fully solid 2D square or triangle because all that empty space, all those holes you punched out. 

## 00:09:39 Speaker 1 

So it literally occupies a fractional space between dimensions. 

## 00:09:43 Speaker 2 

A fractal space, yes. 

## 00:09:45 Speaker 2 

But for a computer architect, that specific number, 1.585, is absolute magic. 

## 00:09:52 Speaker 1 

Why? does a weird decimal number like that help me compute anything? 

## 00:09:56 Speaker 2 

Because it fundamentally changes the connectivity. The proposal cites something called Theorem 2.1 from their preliminary research. 

## 00:10:04 Speaker 1 

Let's dig into Theorem 2.1. 

## 00:10:06 Speaker 2 

It argues that this specific fractal structure allows for hierarchical coupling. 

## 00:10:12 Speaker 1 

Translate hierarchical coupling for us. 

00:10:13 Speaker 2 

Remember the grid commute? 

00:10:15 Speaker 2 

Taking the surface streets. 

00:10:16 Speaker 1 

Yeah, stopping at every intersection. 

## 00:10:18 Speaker 2 

In the fractal, because of how those smaller and larger triangles touch at their corners, you get a hierarchy of pathways. You can jump from a small local neighborhood of quibbits directly to a massive chip-wide expressway almost instantly. 

## 00:10:31 Speaker 1 

Oh, I see. 

## 00:10:32 Speaker 2 

It creates inherent geometric shortcuts through the information space. 

## 00:10:35 Speaker 1 

So instead of hopping linearly through 100 neighbor quibbits, I take one hop to a corner hub and suddenly I'm on the other side of the chip. 

## 00:10:42 Speaker 2 

Effectively, yes. 

## 00:10:44 Speaker 2 

It transforms the topology of the whole network. It's like replacing a congested local road network with a system of wormholes that connect distant neighborhoods seamlessly. 

## 00:10:53 Speaker 1 

And the math in this RFIX proposal claims that this structure... 

## 00:10:58 Speaker 1 

causes the accessible state space to just explode. 

## 00:11:01 Speaker 2 

Explode is honestly the only word for it. 

## 00:11:04 Speaker 1 

I'm looking at the table in their executive summary right now, and the numbers are, I mean, they're hard to believe. They compare a standard linear grid chip directly to this fractal chip. 

## 00:11:14 Speaker 2 

Look at the row for 12 quibits specifically. 

## 00:11:17 Speaker 1 

Okay, so for a standard chip with 12 quibits, the accessible state space is listed as 4,096. 

00:11:23 Speaker 2 

Which is standard binary scaling. Two to the power of 12. That makes total sense. 

00:11:27 Speaker 1 

Right, classical. 

## 00:11:28 Speaker 1 

math. But for the fractal chip with the exact same 12 quibits, it says the accessible space is equivalent to 5.2 times 10 to the 7th. 

00:11:35 Speaker 2 

Which is over 52 million states. 

## 00:11:37 Speaker 1 

52 million states from 12 physical quibits. That's a 10,000-fold advantage. 

00:11:41 Speaker 2 

With the exact same number of physical particles. 

## 00:11:44 Speaker 1 

How is that even physically possible? I mean, 12 quibits are 12 quibits. How can they suddenly hold 50 million states just because you arrange them in a triangle with holes in it? 

00:11:52 Speaker 2 

## Because 

00:11:54 Speaker 2 

It's not that the quibits themselves hold more bits individually. It's that the connections between them create a much, much richer web of possibilities. In a line, quibit A talks to quibit B. Simple enough. But in a fractal, quibit A talks to quibit B, but also instantly to quibit C, and through that hierarchy, instantly to quibit Z. The density of entanglement that you can physically support, the sheer complexity of the quantum state, you can build scales, 

## 00:12:20 Speaker 2 

with that Hausdorff dimension of 1.585. 

## 00:12:23 Speaker 1 

So you are basically unlocking the empty space between the particles. 

00:12:27 Speaker 2 

You are mining the geometry itself for raw computing power. 

## 00:12:30 Speaker 1 

And it gets even wilder as you scale up. I'm looking further down this chart. At 100 qubits, the advantage is listed as 10 to the 95th power. 

## 00:12:37 Speaker 2 

Which for context is a number vastly larger than the number of atoms in the observable universe. 

## 00:12:42 Speaker 1 

From 100 qubits. 

00:12:43 Speaker 2 

From 100 qubits. 

00:12:44 Speaker 1 

So if this math holds up in reality, you wouldn't need a 10 million qubit machine to build a super 

00:12:50 Speaker 1 

supercomputer. You might only need a few 100 perfectly arranged quibbits. 

00:12:54 Speaker 2 

And then that is the core argument of RFX. It's connectivity over quantity. They're 

essentially saying we don't need to build a massive power-hungry engine. We just need to build a much smarter car. 

00:13:05 Speaker 1 

But I have to ask the skeptics question here. Is this just math on a napkin? 

00:13:09 Speaker 2 

It's a fair question. 

00:13:10 Speaker 1 

Because I can write down infinite numbers on a napkin too, but it doesn't mean I can build it. 

00:13:14 Speaker 2 

Right. And this proposal is currently completely honest about its stage. It's at TRL2 

00:13:20 Speaker 2 

technology readiness level 2. 

00:13:22 Speaker 1 

Which means? 

00:13:23 Speaker 2 

Which means concept and formulation is very early days. 

00:13:26 Speaker 1 

Okay. 

00:13:27 Speaker 2 

However, they haven't just drawn triangles. They've run deep simulations. 

00:13:31 Speaker 1 

Real software simulations. 

00:13:32 Speaker 2 

Yes, they used IBM's own quantum software, Quiskit, to model a 5 qubit fractal system. 

00:13:38 Speaker 1 

And what did IBM's software say about it? 

00:13:40 Speaker 2 

Even at that tiny scale, just five virtual quibits, 

00:13:44 Speaker 2 

they saw a 7.1x advantage in the state space compared to a linear chain. The simulation matched their theoretical theorem 2.1 prediction almost perfectly. 

## 00:13:55 Speaker 1 

So the theory actually holds water in the simulator. 

00:13:58 Speaker 2 

It does. It's not just a daydream. 

## 00:14:01 Speaker 2 

The physics engine suggests that this fractional geometry really does unlock extra physical room for data. 

## 00:14:08 Speaker 1 

And they are predicting that if they can build a 12 quibit physical demo, they will hit that 10,000x advantage in the real world. 

## 00:14:16 Speaker 2 

Which is one of the primary milestones of this 18-month funding request, yes. 

## 00:14:20 Speaker 1 

Okay, so that's pillar one, the fractal shortcut. We get massive computing space without needing millions of physical quibits. 

## 00:14:27 Speaker 2 

Right. 

## 00:14:27 Speaker 1 

But, and there's always a, but space doesn't matter if the quibits are still full of noise. 

00:14:31 Speaker 2 

Exactly. 

## 00:14:31 Speaker 1 

I mean, you can have a giant library with infinite shelves, but if all the books on those shelves are written in pure gibberish because of noise, the library is useless. 

00:14:40 Speaker 2 

Correct. Which brings us to the second massive problem, error correction. 

00:14:44 Speaker 1 

And the second pillar of the proposal. 

00:14:48 Speaker 2 

The neglected. 

## 00:14:48 Speaker 1 

I love that name. Like I said, it sounds like a character from the Transformers that nobody respects. Oh, that's just neglecting. Ignore him. 

00:14:54 Speaker 2 

It's actually an incredibly descriptive name when you understand the physics. 

00:14:58 Speaker 1 

Okay, defend the neglecting for us. 

## 00:15:00 Speaker 2 

So this deals with what we call topological quantum field theory, or TQFT. 

00:15:05 Speaker 1 

We are definitely getting into the deep water now. Keep this accessible for us. 

00:15:08 Speaker 2 

Yeah. 

00:15:09 Speaker 1 

What is TQFT in plain English? 

00:15:12 Speaker 2 

Okay. Imagine you have a single piece of string resting on a table. 

00:15:16 Speaker 1 

All right. String on a table. 

00:15:17 Speaker 2 

If you lay it perfectly straight and I bump the table, the string moves, the shape changes instantly. The original information, the straightness is lost. 

00:15:26 Speaker 1 

Okay. That's a standard noisy quibbit. 

00:15:29 Speaker 2 

Exactly. It is highly vulnerable to local errors. A tiny bump ruins it. 

## 00:15:33 Speaker 1 

So a standard quibbit is a loose string. 

00:15:36 Speaker 2 

Now, imagine I take that same string and tie it into a very complex, tight knot. 

00:15:41 Speaker 1 

Okay. 

00:15:41 Speaker 2 

If I bump the table now, the string moves, but the knot is still there. 

## 00:15:46 Speaker 2 

The knot doesn't untie itself just because of vibration. 

## 00:15:49 Speaker 1 

I see. The information is encoded in the actual shape of the knot. 

## 00:15:52 Speaker 2 

Yes, the global topology. To destroy that information, you can't just bump it. You have to physically untie it or take a pair of scissors and cut the string. 

## 00:16:02 Speaker 1 

So the knot itself protects the data. The mathematical structure is the physical shield. 

00:16:06 Speaker 2 

That is topological quantum computing in a nutshell. It is inherently robust against noise. 

## 00:16:14 Speaker 2 

You don't need the 10,000 backup singers like you do with surface codes, because your lead singer is basically wearing armor. 

## 00:16:20 Speaker 1 

The error correction is just built into the physics of the particle. 

## 00:16:23 Speaker 2 

Built right in. 

## 00:16:24 Speaker 1 

So the obvious question from anyone listening is, why aren't we all using this right now? If knots are better than loose strings, 

## 00:16:32 Speaker 1 

Why are Google and IBM spending billions on loose strings? 

## 00:16:35 Speaker 2 

Because the math required to manipulate these knots is incredibly hard, and the actual physical particles we need to create these knots are incredibly elusive in nature. 

## 00:16:44 Speaker 1 

What are the particles called? 

## 00:16:45 Speaker 2 

They're generally called enions, and they are very, very hard to isolate. 

## 00:16:49 Speaker 1 

Okay. 

## 00:16:50 Speaker 2 

Now, most researchers who do look into this typological approach use what are called semi-simple theories. Semi-simple. Right, things involving what are known as Fibonacci enions. 

## 00:16:59 Speaker 1 

And Arfix says that's not good enough. 

## 00:17:02 Speaker 2 

Arfix wants to go much further. They propose using non-semi-simple TQFT. 

## 00:17:07 Speaker 1 

Non-semi-simple. 

## 00:17:08 Speaker 2 

And the star player in that specific theoretical framework is our friend, the Neglectum. 

00:17:15 Speaker 1 

All right, let's look at the specs for this thing. 

## 00:17:18 Speaker 1 

The source material says a neglecton is a particle with a quantum dimension of 0. 

00:17:24 Speaker 2 

Yes, little d equals 0. 

## 00:17:26 Speaker 1 

I need you to explain that. How does a particle physically exist if it has literally 0 dimension? Is it just imaginary math? 

## 00:17:34 Speaker 2 

This is definitely the mind-bending part of the proposal. 

## 00:17:37 Speaker 2 

In this specific context, dimension doesn't mean physical height or width or depth. 

00:17:41 Speaker 1 

It doesn't mean 3D space? 

## 00:17:43 Speaker 2 

No. It refers to the quantum dimension, which is essentially a measure of how much the computing space, the Hilbert space, grows when you add that particle to the system. 

00:17:52 Speaker 1 

Usually if you add a particle, the space grows, right? 

## 00:17:55 Speaker 2 

More physical stuff equals more computing space. 

00:17:57 Speaker 1 

But with a neglecting. 

00:17:58 Speaker 2 

With A neglecting, you add it to the system, and the space just does notice. 

00:18:02 Speaker 1 

It doesn't grow at all. 

00:18:03 Speaker 2 

It contributes exactly 0 to the overall dimension. 

## 00:18:07 Speaker 2 

It's like a ghost particle. It enters a crowded room, but the room doesn't get any more crowded. 

00:18:13 Speaker 1 

So it is functionally invisible to the overall system. 

00:18:16 Speaker 2 

Effectively, yes. But, and this is the absolute key to the whole trick, even though it has zero dimension, 

## 00:18:23 Speaker 2 

It has what physicists call non-trivial braiding phases. 

00:18:26 Speaker 1 

Translation, please. 

00:18:28 Speaker 2 

It means that when you move this ghost particle around your real visible data particles. 

00:18:33 Speaker 1 

When you braid the knot. 

## 00:18:34 Speaker 2 

Exactly. When you braid it around them, it changes their state. It performs a mathematical operation on them. 

## 00:18:40 Speaker 1 

Even though it takes up zero space. 

00:18:42 Speaker 2 

Yes. 

## 00:18:43 Speaker 2 

it acts as a perfect invisible operator. 

## 00:18:45 Speaker 1 

So you can use it to do the actual math, but it doesn't take up any room. And more importantly, it doesn't add to the noise overhead. 

## 00:18:52 Speaker 2 

Exactly. Think of it like having a ghost typist at your computer. They're hitting the keys, they're writing the code, but they don't take up a chair in the office, they don't breathe the oxygen, and they don't make any noise that distracts the other workers. 

## 00:19:03 Speaker 1 

That is wild. 

00:19:04 Speaker 2 

And this theoretical trick. 

00:19:07 Speaker 2 

solves a very specific, incredibly expensive bottleneck in quantum computing called magic state distillation. 

00:19:15 Speaker 1 

Magic state distillation. Now it sounds like we're making whiskey. 

00:19:18 Speaker 2 

It's a really great analogy, actually. 

00:19:20 Speaker 1 

Oh, really? 

00:19:20 Speaker 2 

Yeah, because in standard quantum computing, there is a specific type of calculation called a T-gate. 

00:19:26 Speaker 1 

T-gate. 

## 00:19:26 Speaker 2 

Right. It's a crucial operation for almost all advanced algorithms. You need it. 

## 00:19:31 Speaker 2 

But it is really, really hard to do fault tolerantly with standard qubits. 

00:19:36 Speaker 1 

That's messy. 

## 00:19:37 Speaker 2 

Very messy. So to do a T gate cleanly, you have to create these things called magic states. And the process of making them requires huge resources to quote unquote distill. 

## 00:19:48 Speaker 2 

one clean, usable state from a massive batch of noisy, dirty ones. 

## 00:19:52 Speaker 1 

Like distilling a huge vat of mash to get one pure bottle of whiskey. 

## 00:19:56 Speaker 2 

Exactly. 

00:19:57 Speaker 1 

How huge of a vat are we talking about here? 

## 00:19:59 Speaker 2 

We're talking about needing something like 10 to the 9th operations, a billion background operations, just to clean up the data enough to perform one single T-gate calculation. 

## 00:20:08 Speaker 1 

A billion operations just to prepare for the real operation. 

## 00:20:11 Speaker 2 

Yes. 

00:20:12 Speaker 2 

It's incredibly wasteful. 

## 00:20:13 Speaker 2 

To use the whiskey analogy, it's like needing to build a whole refinery just to get enough fuel to drive your car one block. 

## 00:20:21 Speaker 2 

You are spending 99% of your energy and time just refining the fuel. 

## 00:20:26 Speaker 1 

And the neglectin fixes this refinery bottleneck. 

## 00:20:29 Speaker 2 

Because neglectins can braid in this non-semi-simple way. 

## 00:20:33 Speaker 2 

They provide a universal gate set without any distillation. 

## 00:20:37 Speaker 1 

No refinery needed. 

## 00:20:38 Speaker 2 

None. 

## 00:20:38 Speaker 1 

Yeah. 

## 00:20:38 Speaker 2 

You just take your neglect in, you braid it around your data particles, and the T-gate calculation just magically happens perfectly. 

## 00:20:45 Speaker 1 

So we completely cut out that massive billion operation overhead. 

## 00:20:48 Speaker 2 

Entirely. 

00:20:49 Speaker 1 

The proposal actually mentions a specific statistic here regarding Shor's algorithm. 

## 00:20:54 Speaker 2 

Right, which is the big one? 

## 00:20:55 Speaker 1 

For those listening, Shor's algorithm is the famous mathematical formula used to break encryption. 

## 00:21:00 Speaker 2 

Specifically RSA encryption, which is what protects your credit card online. 

## 00:21:04 Speaker 2 

your banking apps, secure communications. 

## 00:21:06 Speaker 1 

Usually to break RSA with a standard quantum computer, you would need billions of operations, specifically because of that distillation overhead we just talked about. 

## 00:21:15 Speaker 1 

But the Orphix proposal claims that by using these neglectons, you drop that requirement from 10 to the 9th power all the way down to 10 to the 7th. 

## 00:21:26 Speaker 2 

That's 100-fold reduction in base complexity. 

## 00:21:28 Speaker 1 

That is a massive drop. 

00:21:29 Speaker 2 

It really is. 

## 00:21:31 Speaker 2 

In terms of overall gate overhead across the whole system, they cite... 

## 00:21:35 Speaker 2 

A 16x pure reduction in total hardware resources needed. 

## 00:21:40 Speaker 1 

Which fundamentally alters the timeline. 

00:21:42 Speaker 2 

It brings the timeline for a machine that can actually break global encryption way, way forward. 

## 00:21:48 Speaker 2 

It turns a maybe our grandkids will see this in 50 years problem into a we might see this in 10 years problem. 

## 00:21:55 Speaker 1 

Which is incredibly exciting for science, but absolutely terrifying for global cybersecurity. 

## 00:22:00 Speaker 2 

Depending on which side of the firewall you're standing on. 

00:22:02 Speaker 1 

Yeah, exactly. 

## 00:22:03 Speaker 1 

Okay, so let's recap where we are before we move. 

## 00:22:05 Speaker 1 

move to the hardware. 

## 00:22:05 Speaker 1 

Pillar one, fractals give us a massive expansion in computing space without adding physical quibits. 

## 00:22:11 Speaker 1 

Pillar 2, neglectings give us clean, perfect logic operations without that billion step error correction overhead. 

## 00:22:17 Speaker 1 

But as we keep saying, all of this is just abstract math on a whiteboard until you can physically build it. 

## 00:22:23 Speaker 2 

You can't just close your eyes and wish a neglecting into existence. 

00:22:25 Speaker 1 

Right, you need a physical trap to hold these exotic things. 

00:22:29 Speaker 2 Which brings us directly to pillar 3. 

00:22:32 Speaker 2 trapping light. 00:22:33 Speaker 1 

This is the actual hardware part of the RFIX proposal because they aren't proposing to build this out of standard silicon wafers or superconducting metal loops like everyone else. 

00:22:44 Speaker 2 

No. 

00:22:44 Speaker 2 

They are building this entirely with photonics. 

00:22:47 Speaker 1 Light. 

00:22:47 Speaker 2 Light. 00:22:48 Speaker 2 Specifically light inside fused silica. 

00:22:51 Speaker 1 

Which is high-grade, highly pure glass. 

00:22:53 Speaker 2 

Very pure glass, yes. 00:22:54 Speaker 2 

And they are proposing to use lasers to carve a highly specific pattern deep into this glass. 00:22:59 Speaker 1 

The document calls it a hexagonal resonant lattice. 

00:23:03 Speaker 3 

Right. 

00:23:03 Speaker 1 

So imagine a microscopic honeycomb. 

00:23:06 Speaker 2 

Think of a honeycomb, but with the very specific symmetry mathematically called C6V. 

00:23:11 Speaker 1 

C6V. 

00:23:12 Speaker 2 

Yes. 

00:23:13 Speaker 2 

It essentially looks like 19 tiny circles arranged in a very specific geometric cluster inside the glass. 

00:23:18 Speaker 1 

Okay, but why this specific honeycomb shape? 

00:23:21 Speaker 1 

What does 19 circles do to a beam of light? 

00:23:23 Speaker 2 

They're trying to create what is known in physics as a band gap. 

00:23:28 Speaker 1 

A band gap. 

00:23:29 Speaker 1 

Now, I've heard this term in standard electronics. 

00:23:32 Speaker 1 

Right. 

00:23:32 Speaker 2 

Semiconductors. 

00:23:33 Speaker 1 

Semiconductors have band gaps. 

00:23:35 Speaker 1 

As I understand it, that's what makes a computer transistor work. 

## 00:23:39 Speaker 1 

It stops the electricity from flowing until you specifically want it to. 

## 00:23:43 Speaker 2 

It acts as a gate, and it's the exact same fundamental principle here, but applied to photons instead of electrons. 

00:23:50 Speaker 1 

So a band gap for light. 

00:23:52 Speaker 2 

Think of it as this trick no-fly zone for certain frequencies of light. 

## 00:23:56 Speaker 2 

If you engineer this honeycomb lattice perfectly, 

## 00:23:59 Speaker 2 

And the Orphix proposal specifies they're aiming for a quote, complete TM band gap of 21%. 

00:24:06 Speaker 1 

What does 21% mean in practice? 

## 00:24:08 Speaker 2 

It means there is a massive 21% chunk of the light spectrum that literally cannot exist inside that material. 

00:24:14 Speaker 1 

The physics just won't allow it. 

00:24:15 Speaker 2 

Exactly. 00:24:16 Speaker 2 

Yeah. 

00:24:16 Speaker 2 

So if outside quantum noise tries to enter the chip at that band frequency. 

00:24:19 Speaker 1 It just bounces off. 00:24:20 Speaker 2 

It is physically repelled. 00:24:21 Speaker 2 

It cannot propagate through the lattice. 

00:24:23 Speaker 2 

It creates an absolute quiet zone inside the glass. 

00:24:26 Speaker 2 

A fortress. 

00:24:27 Speaker 1 

But wait a second. 

00:24:28 Speaker 1 

If light cannot exist in that 

00:24:29 Speaker 1 

In that space, how do we actually do the computing? 

00:24:32 Speaker 1 

Because we need the light to carry the data, right? 

00:24:35 Speaker 1 

If we just ban all the light, isn't the computer chip just a dark, useless piece of glass? 

00:24:39 Speaker 2 

And that's where the concept of the flat band comes in to save the day. 

00:24:42 Speaker 3 

The flat band. 

00:24:43 Speaker 2 

They engineer a tiny, highly specific exception inside that no-fly zone. 

00:24:48 Speaker 2 

A very specific, narrow frequency where light is allowed to exist, but it's group velocity. 

00:24:54 Speaker 2 

how fast it travels through the material drops to almost zero. 

00:24:57 Speaker 1 

Slow light. 

00:24:58 Speaker 2 

Very slow light. 

00:24:59 Speaker 2 

The spec sheet says VG is less than bunny.01C. 

00:25:04 Speaker 1 

Less than 1% the speed of light. 

00:25:06 Speaker 2 

Yes. 

00:25:06 Speaker 1 

That is incredibly slow for something that usually takes, what, about a single second to get all the way to the moon? 

00:25:11 Speaker 2 

It is functionally crawling. 

00:25:13 Speaker 2 

It's like freezing the photon in place. 

## 00:25:15 Speaker 2 

So we create this geometric fortress where outside noise is totally banned. 

## 00:25:19 Speaker 2 

And inside the safety of that fortress, we have our data-carrying light moving in extreme slow motion so we can easily manipulate it and braid our neglectance around it. 

## 00:25:29 Speaker 1 

That is profoundly elegant. 

## 00:25:30 Speaker 2 

It gets better because this specifically connects back to the fractal shape from pillar one. 

## 00:25:35 Speaker 1 

Oh, how? 

## 00:25:36 Speaker 2 

The proposal mentions A phenomenon called Anderson localization. 

00:25:40 Speaker 1 

Anderson localization. 

00:25:42 Speaker 2 

This deals with the concept of disorder in physics. 

## 00:25:45 Speaker 1 

Usually, disorder like imperfections or messiness in a material is bad for computers. 

00:25:51 Speaker 1 

We spend billions making perfectly pure silicon wafers. 

## 00:25:55 Speaker 2 

Exactly. 

00:25:56 Speaker 1 

But this paper seems to suggest that disorder is good. 

00:26:00 Speaker 2 

It's highly counterintuitive. 

00:26:02 Speaker 2 

In a perfectly orderly standard crystal lattice, 

00:26:06 Speaker 2 

Waves of light or electrons just ripple out forever until they hit an edge. 

00:26:10 Speaker 2 

But if you introduce the right kind of mathematical disorder, or in this specific case, a fractal structure, which fundamentally looks disordered compared to standard grid, because of all those holes. 

00:26:19 Speaker 1 

The holes in the Sierpiski gasket. 

00:26:21 Speaker 2 

Exactly. 

00:26:22 Speaker 2 

The light waves actually get confused. 

00:26:23 Speaker 2 

They bounce off the holes, they destructively interfere with themselves, and they get stuck in place. 

00:26:27 Speaker 1 

They trap themselves. 

00:26:28 Speaker 2 

They localize. 

00:26:30 Speaker 2 

Anderson localization. 

00:26:32 Speaker 2 

Because the fractal has a spectral dimension, 

00:26:35 Speaker 2 

The math says D's equals 1.36, which is strictly less than two. 

00:26:42 Speaker 2 

The light has to localize. 

00:26:43 Speaker 1 

So the fractal structure literally uses its own weird geometry to pin the quantum state in place. 

00:26:48 Speaker 2 

Yes. 

00:26:49 Speaker 2 

It protects the fragile information from leaking away into the rest of the chip. 

00:26:52 Speaker 1 

The key statistic they claim here for this is a 16x improvement in coherence time. 

00:26:58 Speaker 2 

Incoherence time is everything. 

00:27:00 Speaker 2 

It's how long the quantum thought lasts before it disappears into noise. 

00:27:03 Speaker 1 

So 16x longer. 

00:27:04 Speaker 2 

16x is. 

00:27:05 Speaker 2 

is an absolute eternity in quantum mechanics. 

## 00:27:08 Speaker 2 

It means you can do 16 times as much math, string together 16 times as many operations before the system naturally crashes. 

## 00:27:14 Speaker 1 

So it's this perfectly interlocked trinity of concepts. 

## 00:27:17 Speaker 1 

Fractals give you the immense space, neglectins give you the error-free logic, and photonic band gaps give you the physical protection. 

## 00:27:23 Speaker 2 

It is a beautiful, beautiful theoretical sandwich. 

## 00:27:25 Speaker 1 

But there's always a but with these deep dives. 

## 00:27:27 Speaker 2 

The but is that it's all still on paper, or it's in a Quiscit simulation. 

## 00:27:32 Speaker 2 

They have not built this full integrated supercharge. 

## 00:27:35 Speaker 2 

yet. 

## 00:27:36 Speaker 2 

This document is a proposal for funding, not a product brochure you can order from. 

## 00:27:41 Speaker 1 

And that is exactly why they are asking for the money. 

## 00:27:45 Speaker 1 

Let's look at their actual 18-month battle plan. 

00:27:47 Speaker 3 

You're right about it. 

## 00:27:48 Speaker 1 

They are asking for $1.25 million to make this a reality, which honestly, for a project that claims it can essentially revolutionize global computing and break RSA encryption. 

## 00:28:00 Speaker 2 

It seems ridiculously cheap, doesn't it? 

00:28:02 Speaker 1 

Really does. 

00:28:03 Speaker 1 

I mean, Google probably spends $1.25 million 

## 00:28:05 Speaker 1 

million just on catering for their quantum team in a year. 

## 00:28:08 Speaker 2 

It is a remarkably capital efficient proposal. 

## 00:28:12 Speaker 2 

And that's primarily because they aren't trying to build the entire finished computer right now. 

## 00:28:16 Speaker 2 

They are just trying to rigorously validate the underlying physics. 

## 00:28:20 Speaker 1 

As you said, moving from TRL 2 to TRL 4 lab validations. 

## 00:28:23 Speaker 2 

True of it works on a bench before you try to scale it. 

## 00:28:25 Speaker 1 

So let's walk the listener through these protocols. 

## 00:28:27 Speaker 1 

They have four very specific experiments planned. 

## 00:28:30 Speaker 1 

If they get this check from the grant committee, this is exactly what they're going to do. 

## 00:28:33 Speaker 2 

Protocol one. 

## 00:28:34 Speaker 1 

Right, protocol one, the diamond test. 

## 00:28:36 Speaker 2 

This is meant to test the coherence, the trapping light part of the theory. 

## 00:28:41 Speaker 2 

They're going to use something called nitrogen vacancy centers in diamond. 

00:28:45 Speaker 1 

NV centers. 

## 00:28:46 Speaker 1 

Are these just little microscopic defects in a physical diamond that act like qubits? 

00:28:50 Speaker 2 

Yes. 

## 00:28:51 Speaker 2 

Basically, you take out two carbon atoms and put in a nitrogen atom and a hole. 

## 00:28:57 Speaker 2 

They are a very standard, incredibly robust platform for testing out new quantum theories. 

00:29:02 Speaker 1 

Okay, so what do they do with the diamond? 

## 00:29:04 Speaker 2 

Orphix wants to take this diamond, use a technique called e-beam lithography. 

## 00:29:08 Speaker 1 

E-beam lithography. 

## 00:29:09 Speaker 2 

Which is essentially using a highly focused electron beam as a fancy atomic level chisel. 

## 00:29:16 Speaker 2 

And they want to carve that Sierpatowski fractal pattern, the holes, directly into the physical surface of the diamond. 

## 00:29:21 Speaker 1 

So they are physically manufacturing the fractal. 

00:29:24 Speaker 1 

They're taking a real diamond and etching millions of tiny triangles into it. 

00:29:27 Speaker 2 

Physically making it, yes. 

00:29:29 Speaker 2 

And the goal of protocol one is to prove that the fractal shape alone naturally boosts the coherence time of the quivits. 

00:29:36 Speaker 1 

And their predicted numbers. 

00:29:37 Speaker 2 

They predict they will see a jump from a standard 50 microseconds of coherence all the way up to 160 microseconds. 

00:29:44 Speaker 1 

That's a three-point 

00:29:46 Speaker 1 

2x boost in lifespan just from carving a geometric shape around the particle. 

00:29:51 Speaker 2 

Just from the geometry. 

00:29:52 Speaker 2 

And the budget for this specific test... 

00:29:54 Speaker 1 

It says here about $115,000. 

00:29:55 Speaker 2 

Which is relatively cheap because they are using well-understood standard fabrication techniques on a standard material. 

00:30:03 Speaker 1 

Okay, moving on to protocol 2, the ion trap. 

## 00:30:07 Speaker 1 

Now looking at the budget breakdown, this looks like the single most expensive part of the whole menu. 

00:30:11 Speaker 2 

It is $395,000. 

00:30:14 Speaker 1 

Almost 1/3 of the entire budget. 

00:30:16 Speaker 2 

Yes. 

## 00:30:17 Speaker 2 

And this one is designed to prove the fractal advantage, that massive memory expansion we talked about back in part one, the 10,000X boost. 

## 00:30:24 Speaker 1 

But wait, why are they using ions for this? 

## 00:30:26 Speaker 1 

Why not just stick with the diamond from protocol one? 

00:30:28 Speaker 2 

Because trapped ions are physically movable. 

00:30:31 Speaker 1 

Well, suvable. 

00:30:31 Speaker 2 

Yes. 

## 00:30:32 Speaker 2 

You can use complex electric fields to literally shuttle the individual ions around inside a vacuum chamber. 

## 00:30:38 Speaker 1 

Oh, I see. 

## 00:30:39 Speaker 2 

So if you want to perfectly simulate a fractal connection where quibit A needs to talk to quibit Z instantly across the hierarchy, 

## 00:30:46 Speaker 2 

You can physically move the ions in real time to mimic that exact geometry. 

## 00:30:52 Speaker 1 

You can choreograph a literal fractal dance with the particle. 

## 00:30:55 Speaker 2 

Precisely. 

## 00:30:56 Speaker 2 

They are proposing to go to Sandia National Labs, which is bringing out the big guns, to use their trapped ytterbium-171 ions. 

## 00:31:04 Speaker 1 

Ytterbium-171. 

## 00:31:05 Speaker 2 

Right. 

00:31:06 Speaker 2 

And the goal here is to finally verify theorem 2.1 in the physical world. 

## 00:31:10 Speaker 1 

That's the theorem that says 12 quibits can act like 52 million states. 

## 00:31:14 Speaker 2 

Exactly. 

## 00:31:15 Speaker 2 

They want to prove that with just 12 physical ions arranged in this dynamic fractal way, they can access that 10,000x larger state space. 

## 00:31:24 Speaker 1 

I mean, if they pull that off. 

00:31:25 Speaker 2 

It changes everything. 

00:31:26 Speaker 1 

Proving you can get supercomputer level state space out of 12 measly particles. 

00:31:30 Speaker 1 

That's the cover of Nature Magazine right there. 

## 00:31:32 Speaker 2 

That is the complete validation of the Hilbert space scaling. 

## 00:31:35 Speaker 2 

If that Quiskit chart turns out to be real, physical, measurable data, and not just a simulation quirk, the entire quantum industry will have to stop what they are doing and pay attention. 

## 00:31:44 Speaker 1 

Because it fundamentally breaks the core assumption that N physical quibits equals N computing power. 

## 00:31:50 Speaker 2 

It proves that the shape matters more than the count. 

## 00:31:52 Speaker 1 

Wow. 

00:31:53 Speaker 1 

Okay. 

00:31:54 Speaker 1 

Protocol 3, the blast chip. 

## 00:31:56 Speaker 2 

This is specifically testing the band gap, the photonic fortress. 

00:32:00 Speaker 1 

Right, the fused silica. 

00:32:02 Speaker 1 

And looking at the budget, this one is remarkably cheap. 

00:32:05 Speaker 2 

About $48,000. 

00:32:06 Speaker 1 

Why is it so much cheaper than the ion trap? 

00:32:09 Speaker 2 

Because the fabrication is much simpler. 

00:32:11 Speaker 2 

They use femtosecond lasers to write the hexagonal lattice directly into the bulk of the glass. 

00:32:16 Speaker 1 

Femtosecond lasers. 

00:32:18 Speaker 2 

It's an incredibly fast, efficient technique. 

00:32:21 Speaker 2 

You don't need a massive sterile clean room for weeks on end. 

## 00:32:24 Speaker 2 

You just take a block of pure glass, you zap it with a laser to change its index of refraction, and it's done. 

00:32:31 Speaker 1 

And what are they looking for when they test it? 

00:32:32 Speaker 1 

The no-fly zone. 

00:32:33 Speaker 2 

Yes. 

## 00:32:34 Speaker 2 

They will shine light through the carved glass and measure if that theoretical 21% band gap actually appears in reality. 

## 00:32:42 Speaker 2 

They also want to observe something called edge states. 

## 00:32:44 Speaker 1 

Edge states. 

## 00:32:45 Speaker 2 

This is a topological feature where the data-carrying light flows perfectly along the physical edge of the glass chip without ever scattering or leaking into the center, even if there are defects. 

## 00:32:56 Speaker 1 

It's basically like a perfect superconductor, but for light instead of electricity. 

00:32:59 Speaker 2 

Exactly. 

00:33:00 Speaker 2 

A lossless highway on the edge of the chip. 

00:33:02 Speaker 1 

All right, finally, protocol 4. 

00:33:04 Speaker 1 

Majorana T-junction. 

00:33:07 Speaker 1 

Now, just reading the description here, this sounds like the absolute riskiest one of the bunch. 

00:33:12 Speaker 2 

It absolutely is. 

00:33:14 Speaker 2 

This connects directly to the neglecting theory. 

00:33:16 Speaker 1 

The ghost particles. 

00:33:17 Speaker 2 

Right. 

00:33:17 Speaker 2 

To try and prove this, they need to use indium antimonide nanowires. 

00:33:21 Speaker 1 

Indium antimonide, that is quite a mouthful. 

00:33:24 Speaker 2 

It's a very exotic, very difficult semiconductor material. 

00:33:29 Speaker 2 

and they are proposing to build a microscopic T-shaped device out of it. 

00:33:33 Speaker 1 

Why T-shaped? 

00:33:34 Speaker 1 

Why not just a straight wire? 

00:33:35 Speaker 2 

Because a T-junction mimics A branching point of a fractal. 

00:33:39 Speaker 1 

Oh, to test the hierarchical movement. 

00:33:40 Speaker 2 

Exactly. 

00:33:41 Speaker 2 

They want to force particles through that junction to demonstrate those complex braiding operations we talked about. 

## 00:33:47 Speaker 1 

This is where they try to actually prove the topological protection of the knots. 

## 00:33:51 Speaker 2 

Yes. 

## 00:33:52 Speaker 2 

They are looking for a measurable 1.5x boost in fidelity accuracy during braiding. 

## 00:33:57 Speaker 2 

But the massive risk here, which they are very upfront about, is something called quasiparticle poisoning. 

## 00:34:04 Speaker 1 

Quasi-particle poisoning, which honestly sounds like something that happens to Superman in a comic book. 

## 00:34:09 Speaker 1 

Look out, Clark, it's quasi-particle poisoning. 

## 00:34:12 Speaker 2 

It does sound like sci-fi. 

## 00:34:14 Speaker 2 

But in reality, it basically means stray, low-level energy from the environment accidentally breaks the incredibly delicate superconducting state of the nanowire. 

## 00:34:23 Speaker 1 

And if that happens? 

## 00:34:24 Speaker 2 

The wire stops being superconducting, the topological state collapses, and the experiment just fails outright. 

00:34:30 Speaker 1 

And the proposal actually lists this up front as a high-risk, high-impact problem. 

00:34:34 Speaker 2 

They do. 

00:34:34 Speaker 2 

They know it's a moonshot. 

00:34:35 Speaker 1 

But they do have a built-in fallback strategy, right? 

00:34:38 Speaker 1 

They aren't putting all 1.2. 

00:34:40 Speaker 1 

to $5 million into the poisonous basket. 

00:34:42 Speaker 2 

No, they are very strategic. 

## 00:34:44 Speaker 2 

They explicitly state that if the Majorana nanowire stuff gets bogged down in fabrication issues or poisoning, they will immediately shift that chunk of funding over to accelerate the diamond and glass protocols. 

## 00:34:56 Speaker 1 

Because those are much more mature, reliable technologies. 

00:35:00 Speaker 2 

Exactly. 

00:35:01 Speaker 2 

It's a very smart hedge for a grant application. 

## 00:35:04 Speaker 2 

It shows the reviewers that they are realistic about the cutting-edge physics risks. 

## 00:35:08 Speaker 1 

So that is the 18-month plan, $1.25 million, for highly specific experiments. 

## 00:35:15 Speaker 1 

Let's zoom out for a second and look at the big picture. 

00:35:17 Speaker 1 

Who exactly are they fighting against in the market? 

00:35:20 Speaker 1 

We mentioned IBM and Google earlier. 

00:35:22 Speaker 2 

The competitive landscape is incredibly fierce right now. 

00:35:25 Speaker 2 

IBM and Google are the unquestioned titans of the industry. 

00:35:29 Speaker 2 

They are entirely committed to the brute force approach. 00:35:31 Speaker 2 

They are scaling up flat, superconductive. 00:35:34 Speaker 2 Euclidean chips. 00:35:35 Speaker 2 

They have 1000 quiddits now. 00:35:36 Speaker 2 

They're aiming for 10,000 soon. 00:35:38 Speaker 1 

They have the massive budgets, the giant staffs, and all the momentum. 

00:35:41 Speaker 2 

Yes. 

00:35:42 Speaker 2 

But as we discussed, they're hitting the scaling wall hard. 

00:35:46 Speaker 1 

Because of the noise. 

00:35:47 Speaker 2 

They are struggling to get their error rates below 10 to the minus third, which sounds really low. 

00:35:52 Speaker 1 

99.9% accurate. 

00:35:54 Speaker 2 

Right. 

## 00:35:54 Speaker 2 

But in quantum computing, a 0.1% error rate is still way too high. 

00:35:59 Speaker 2 

It still requires that massive 10,000 to 1 overhead. 

00:36:03 Speaker 2 

So IBM 

00:36:04 Speaker 2 

and Google are running face-first into the sheer physical limits of wiring density and cryogenic cooling. 

00:36:10 Speaker 1 

Then you have companies like IonQ. 

00:36:12 Speaker 2 

Right, IonQ uses trapped ions. 

00:36:14 Speaker 2 

They actually have extremely high fidelity. 

00:36:16 Speaker 2 

Their quibits are very, very accurate. 

00:36:17 Speaker 1 

So they don't need as much error correction. 

00:36:19 Speaker 2 

They need less, yes. 00:36:21 Speaker 2 

But their fatal flaw right now is that they are slow. 

00:36:24 Speaker 1 

Slow to compute. 

00:36:25 Speaker 2 

Their gate operations take microseconds to complete, whereas superconducting chips take nanoseconds. 

00:36:31 Speaker 1 

So they are accurate, but sluggish. 

00:36:32 Speaker 2 

Exactly. 

00:36:33 Speaker 1 

And then there's Orphix. 

00:36:34 Speaker 2 

Orphix is the massive underdog here. 

00:36:36 Speaker 2 

They're sitting at TRL2. 

00:36:37 Speaker 2 

They don't have a glossy product yet. 

00:36:40 Speaker 2 

But their core argument to the industry is essentially, you guys are trying to build 100 story skyscraper on a swamp using brute force. 

00:36:48 Speaker 2 

We are suggesting we pause, move to solid rock, and use better architecture. 

00:36:53 Speaker 1 

This is the effective quibit argument from the paper. 

## 00:36:55 Speaker 2 

Yes, the effective quibit. 

## 00:36:57 Speaker 1 

They argue that one single or fixed quibit, because it is embedded in a fractional shortcut and protected by a photonic band gap and operated by a neglectin, is practically worth 100 or maybe even 1000 standard IBM quibits. 

## 00:37:12 Speaker 2 

It is entirely about quality over quantity, smarter geometry over sheer brute force scale. 

## 00:37:19 Speaker 1 

But what are the real risks if this gets funded, aside from the Superman poisoning we talked about? 

## 00:37:23 Speaker 2 

Fabrication is the biggest hurdle by far. 

## 00:37:26 Speaker 2 

Making perfect fractals at the nanoscale is absurdly difficult. 

## 00:37:30 Speaker 1 

Because you're dealing with individual atoms. 

00:37:32 Speaker 2 

Exactly. 

## 00:37:33 Speaker 2 

The proposal itself admits they need less than a 5% physical variation in the lattice structure. 

## 00:37:37 Speaker 1 

Less than 5%. 

## 00:37:38 Speaker 2 

If your tiny carved triangles are even slightly lopsided or the holes aren't perfectly spaced, 

00:37:44 Speaker 2 

The mathematical fractal magic just breaks down. 

00:37:46 Speaker 1 

The Anderson localization relies on the math being absolutely precise. 

00:37:50 Speaker 2 

Perfectly precise. 

## 00:37:51 Speaker 2 

If the physical geometry is off by a few nanometers, the light doesn't trap. 

## 00:37:55 Speaker 2 

The geometric shortcut becomes a dead end. 

## 00:37:57 Speaker 1 

So if the manufacturing isn't literal perfection, the whole fortress leaks. 

00:38:02 Speaker 2 

Exactly. 

00:38:03 Speaker 2 

And furthermore, relying on neglectins, these zero-dimension particles that are theoretically beautiful but experimentally elusive, is a massive scientific gamble. 

00:38:14 Speaker 2 

We haven't fully confirmed these non-semi-simple dynamics in a physical lab environment yet. 

00:38:18 Speaker 1 

But just for a moment, if it works, let's dream for a second. 

00:38:22 Speaker 1 

If Orfix pulls this off, what are the broader impacts for the world? 

00:38:26 Speaker 2 

Well, the first major impact is the scary one we touched on, cryptanalysis. 

## 00:38:31 Speaker 1 

Breaking encryption. 

## 00:38:32 Speaker 2 

Breaking RSA encryption. 

00:38:34 Speaker 2 

The entire security of the modern internet relies on the fact that factoring huge numbers into primes is virtually impossible for classical computers. 

## 00:38:42 Speaker 2 

Shor's algorithm on a quantum computer makes it easy, but as we said, currently you need billions of operations. 

## 00:38:48 Speaker 2 

Orfix says they can do it with millions. 

## 00:38:50 Speaker 1 

Which takes the timeline for breaking the internet security and shrinks it drastically. 

00:38:55 Speaker 2 

It makes a machine capable of breaking RSA, physically feasible, with much, much smaller, cheaper hardware. 

## 00:39:02 Speaker 2 

It takes the quantum threat from a distant, abstract worry for future generations and turns it into a near-term reality that banks and governments have to prepare for right now. 

00:39:13 Speaker 1 

That is genuinely the scary part. 

00:39:15 Speaker 1 

What's the hopeful part? 

00:39:16 Speaker 1 

What's the good news if this works? 

00:39:18 Speaker 2 

Simulation. 

00:39:19 Speaker 2 

Specifically quantum chemistry. 

00:39:21 Speaker 1 

Okay. 

## 00:39:21 Speaker 2 

If you really do have a state space advantage of 10 to the 95th power, you can perfectly simulate complex chemical molecules that are completely impossible for current computers to model. 

00:39:32 Speaker 1 

Like what kind of molecules? 

## 00:39:34 Speaker 2 

We're talking about designing radically new, perfectly efficient catalysts for atmospheric carbon capture, or discovering true room temperature superconductors that would revolutionize the power grid. 

## 00:39:44 Speaker 1 

Because nature itself operates on quantum rules. 

00:39:47 Speaker 2 

Nature is quantum, yes. 

00:39:49 Speaker 2 

And crucially for this proposal, nature is often fractal. 

00:39:52 Speaker 1 

Oh, that's true. 

00:39:53 Speaker 2 

Look at the shape of a fern leaf. 

00:39:55 Speaker 2 

Look at the jagged line of a coastline. 

00:39:57 Speaker 2 

Look at the branching bronchia inside your own lungs. 

00:39:59 Speaker 1 

They are all fractals. 

00:40:01 Speaker 2 

They are. 

00:40:02 Speaker 2 

It might just be that a fractal computer architecture is fundamentally better at simulating A fractal world. 

00:40:09 Speaker 1 

That is a highly poetic thought for a physics paper. 

00:40:11 Speaker 2 

It really is. 

00:40:12 Speaker 1 

And commercially, what's the end game for the company? 

00:40:15 Speaker 1 

The proposal mentions licensing. 

00:40:17 Speaker 2 

Yeah, they are very realistic. 

00:40:18 Speaker 2 

They aren't necessarily trying to manufacture millions of chips and kill IBM. 

00:40:22 Speaker 2 

They want to validate the physics, patent the architecture, and then license it to IBM and Google. 

00:40:28 Speaker 2 

Imagine an IBM superconducting chip, but the quibits are arranged in a Sierpiski pattern instead of a grid. 

00:40:35 Speaker 2 

That's Orphix's year three to five goal. 

00:40:38 Speaker 2 

They want to sell the revolutionary map, not necessarily manufacture the car. 

00:40:41 Speaker 1 

It's a total paradigm shift. 

00:40:43 Speaker 2 

It really, really is. 

00:40:44 Speaker 2 

We have spent decades so hyper-focused on the physical nature of the particle. 

00:40:48 Speaker 2 

Is it a trapped ion? 

00:40:50 Speaker 2 

Is it a photon? 

00:40:51 Speaker 2 

Is it a loop of superconducting current? 

00:40:53 Speaker 1 

Right. 

00:40:53 Speaker 2 

And RFX is essentially saying it almost doesn't matter what the particle is. 

00:40:57 Speaker 2 

It matters where you place it. 

00:40:58 Speaker 1 

The shape of the computer is as important as the computer itself. 

00:41:01 Speaker 2 

Exactly. 

00:41:02 Speaker 1 

Yeah. 

00:41:02 Speaker 2 

There's a quote from the abstract that really stuck with me. 

00:41:05 Speaker 2 

Fractal Hilbert space allows shortcuts through information space. 

00:41:08 Speaker 1 

Shortcuts through information space. 

00:41:11 Speaker 1 

So what should we, and you listening, be watching out for? 

00:41:14 Speaker 1 

This is an 18-month plan starting, well, hypothetically right now. 

00:41:18 Speaker 1 

What's the canary in the coal mine? 

00:41:19 Speaker 2 

Watch closely for protocol 3, the laser writing tests in the glass. 

00:41:24 Speaker 1 

The $48,000 one. 

00:41:25 Speaker 2 

Yes. 

00:41:26 Speaker 2 

That's the cheapest, fastest, most straightforward experiment on their docket. 

00:41:32 Speaker 2 

If we see a peer-reviewed paper come out in the next six months, demonstrating A 21% band gap in a fractal glass chip. 

00:41:40 Speaker 1 

That's the first domino. 

00:41:41 Speaker 2 

If that domino falls, 00:41:43 Speaker 2 

The entire industry is going to pivot. 

00:41:44 Speaker 1 

And the scaling wall might just have a massive crack in it. 

00:41:47 Speaker 2 

A fractal-shaped crack, yeah. 

## 00:41:48 Speaker 1 

This has been an absolute trip of a deep dive. 

## 00:41:52 Speaker 1 

From ghost particles that don't take up space to computer chips made of Swiss cheese geometry. 

00:41:58 Speaker 2 

It's quantum mechanics at its best. 

## 00:41:59 Speaker 1 

It really reminds you that innovation isn't always about brute forcing more power. 

00:42:04 Speaker 1 

Sometimes it's just about drawing a smarter map. 

## 00:42:06 Speaker 2 

Sometimes you just have to look at the exact same problem from a slightly different dimension. 

00:42:10 Speaker 2 

Literally. 

00:42:11 Speaker 1 

Literally. 

00:42:12 Speaker 1 

1.5 

00:42:13 Speaker 1 

85 dimensions to be exact. 

00:42:15 Speaker 1 

I love that. 

00:42:16 Speaker 1 

Well, thank you for unpacking this incredibly dense material with us today. 

00:42:20 Speaker 2 

It was my pleasure. 

00:42:21 Speaker 1 

And to you listening, I want to leave you with a final thought to mull over. 

00:42:25 Speaker 1 

If Orphix is right, if fractal geometry naturally unlocks computational power that standard grids just can't touch, 

## 00:42:33 Speaker 1 

then maybe fractals aren't just a clever engineering hack for quantum computers. 

00:42:37 Speaker 1 

Right. 

00:42:38 Speaker 1 

Maybe it implies something much deeper about the universe. 

00:42:41 Speaker 1 

If nature already uses fractals for everything from snowflakes to blood vessels, maybe the fundamental base code of the universe isn't a rigid Euclidean grid at all. 

## 00:42:51 Speaker 2 

It's topological. 

00:42:51 Speaker 1 

Exactly. 

00:42:52 Speaker 1 

The universe itself might be computing its own existence through fractal geometry. 

00:42:56 Speaker 1 

Next time you see a fern leaf or a jagged coastline, remember, that exact shape might be the master key to the next 

00:43:03 Speaker 1 

era of human supercomputing. 

00:43:05 Speaker 1 

Keep diving deep. 

00:43:06 Speaker 1 We'll see you next time. 

