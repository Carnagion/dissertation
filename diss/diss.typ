#import "@preview/lovelace:0.2.0": *
#import "@preview/timeliney:0.0.1": *

// NOTE: Needs to be called at the top of the document to setup lovelace
#show: setup-lovelace

#set text(font: "EB Garamond", size: 11pt)
#set par(justify: true)

#set heading(numbering: "1.1")
#show heading: set block(above: 2em, below: 1.3em)

#show math.equation.where(block: true): set math.equation(numbering: "(1)")

// NOTE: Workaround to get non-math text to use EB Garamond in math equations until Typst ships a native function for doing so
#let mathtext = math.text.with(font: "EB Garamond", weight: "regular")

#let pseudocode = pseudocode.with(indentation-guide-stroke: 0.1pt)

#set figure(gap: 1em)
#show figure.where(kind: table): set block(breakable: true)
#show figure.where(kind: table): set par(justify: false)

// NOTE: Workaround to make prose citations use "et al" with a lower author count threshold
// TODO: Check if there is a way to already do this in Typst without using a CSL file
#show cite.where(form: "prose"): set cite(style: "ieee-et-al-min.csl")

#let email(email) = link("mailto:" + email, raw(email))

// TODO: Remove when all todos are removed
#let todo(message) = raw("// TODO: " + message, block: true, lang: "rust")

#v(1fr)
#align(center)[
    // TODO: Should the heading be changed to something better?
    // TODO: Experiment with a proper heading instead of larger text size
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    // TODO: Check if "Final Dissertation" is what it should be called
    #text(size: 14pt)[_COMP3003 Final Dissertation_]

    #v(0.2fr)

    // TODO: Check if name and supervisor details should be included
    #stack(dir: ltr, spacing: 1fr)[
        _By_\
        Indraneel Mahendrakumar\
        20372495\
        #email("psyim3@nottingham.ac.uk")\
    ][
        _Supervised By_\
        Geert De Maere\
        Assistant Professor\
        #email("geert.demaere@nottingham.ac.uk")\
    ]

    #v(0.2fr)

    MSci. Hons. Computer Science with Artificial Intelligence\
    University of Nottingham\
    2023-2024
]
#v(1fr)

#pagebreak()

#v(1fr)

#align(center)[
    // TODO: Experiment with a proper heading instead of larger text size
    #text(size: 14pt)[*Abstract*]
]

#todo("Write abstract")

#lorem(100)

#lorem(50)

#v(1fr)

#pagebreak()

#outline(indent: auto)
#pagebreak()

// NOTE: Done after cover page, abstract, and table of contents since we don't want page numbers to show up on them
// TODO: Decide if previous pages should have numbering as well and if they should have a different style of numbering
#set page(numbering: "1")

// NOTE: Forces the page numbering to begin from here rather than from the cover page or abstract page
// TODO: Decide if numbering should begin from previous pages but should not be shown
#counter(page).update(1)

// TODO: Revise all headings as necessary

= Introduction

#todo("Write introduction")

== Background

#todo("Write background")

== Motivation

#todo("Write about motivation")

= Existing Literature

#todo("Write about existing literature")

= Problem Description

Given a set of arrivals $A$ and departures $D$, the runway and de-icing sequencing problem consists of finding a sequence of landing and take-off times as well as a sequence of de-icing times such that an optimal value is achieved for a given objective function, subject to the satisfaction of all hard constraints.

== Constraints

A feasible solution to the problem must meet satisfy separation requirements, hard time windows, earliest take-off times according to CTOT slots, and holdover times.
A sequence that violates these hard constraints is considered to be infeasible, and can thus be eliminated from the solution space.

=== Separation Requirements

Any two consecutive aircraft $i$ and $j$ (where $i$ precedes $j$) are required to have a minimum runway separation $delta_(i j)$ between them, which is determined by their weight classes, speed groups, and (for departures) Standard Instrument Departure (SID) routes.
An aircraft's weight class influences the severity of wake turbulence it causes, the time required for this turbulence to dissipate, and its sensitivity to the wake turbulence caused by other aircraft.
Larger or heavier aircraft typically generate greater turbulence, to which smaller or lighter aircraft are more sensitive.
Consequently, a larger separation may be required when a large aircraft is followed by a small one, than when a small aircraft is followed by a large one @demaere-pruning-rules.

// TODO: Check if we actually need to mention this or can leave it out or shorten it
Similarly, a larger separation may be required when a slow aircraft is followed by a faster one on the same route, to prevent the latter from catching up to the former before their routes diverge.
Separations for SID routes are also influenced by the climb and relative bearing of the route, as well as congestion in downstrea airspace sectors.
The latter factor may require an increased separation upon take-off to space out traffic and prevent the overloading of en-route sectors and controllers @demaere-pruning-rules.

Separation requirements are thus asymmetric.
It is important to note that they also do not necessarily obey the triangle inequality -- given three aircraft $i$, $j$, and $k$, the relation $delta_(i j) + delta_(j k) >= delta_(i k)$ does not necessarily hold.
The landing (or take-off) time of one aircraft can thus be influenced not just by the immediately preceding aircraft, but by all preceding aircraft in the sequence @demaere-pruning-rules.

=== Time Windows

If an aircraft $i$ is subject to a hard time window $T_i$ which is defined by its earliest (start) time $e_i$ and latest (end) time $l_i$, then its landing (or take-off) time $t_i$ _must_ be within this window.
In other words, $e_i <= t_i <= l_i$.

In this model, each aircraft is assumed to be subject to a hard time window, although this is not always the case in the real world.
However, this assumption can be made without loss of generality -- an aircraft $i$ that is not subject to a hard time window can instead be considered to be subject to a very large time window, such that its start time $e_i$ is early enough and its end time $l_i$ late enough so as to never affect solutions in practice @demaere-pruning-rules.

=== Calculated Take-Off Times

In addition to a hard time window, a departure $i$ might be subject to a Calculated Take-Off Time (CTOT) slot $c_i$, during which it should take off.
Typically, a CTOT has a tolerance of -5 to +10 minutes (i.e. 5 minutes before and 10 minutes after $c_i$) and its time window can thus be defined by its earliest (start) time $u_i$ and latest (end) time $v_i$; however, this model makes no such assumptions and allows for customizable CTOT tolerances per departure.

Much like a hard time window, a departure cannot take off before $u_i$, but it _may_ be scheduled after $v_i$ -- although this is heavily penalized.
The start time of a CTOT slot is thus modeled as a hard constraint, while its end time is modeled as a soft constraint.

=== Holdover Times

#todo("Write about holdover times as a constraint")

== Objectives

#todo("Write about objective function")

// TODO: Check if pruning rules such as complete orders and disjoint time windows should be mentioned here
= Implementation

#todo("Write short introduction to different approaches used")

== Model

#todo("Include final mathematical model, objectives, and constraints")

== Branch-and-Bound Program

#todo("Write short introduction to branch-and-bound program")

=== Sequential De-Icing

#todo("Include explanation and pseudocode for sequential de-icing mode")

=== Integrated De-Icing

#todo("Include explanation and pseudocode for integrated de-icing mode")

=== Rolling Horizon Extension

#todo("Include explanation and pseudocode for rolling horizon")

// TODO: Check if this belongs here or is better off somewhere else
== Sequence Visualizer

#todo("Write about visualizer")

= Results

#todo("Write introduction to results")

// TODO:
// - Furini instances
//   - Limited, 10 or 20 flights
//     - Branch-and-bound
//     - CPLEX
//   - Full
//     - Branch-and-bound
//     - CPLEX
// - Heathrow instances
//   - Times in minutes
//     - Branch-and-bound
//     - CPLEX
//   - Times in seconds
//     - Branch-and-bound
//     - CPLEX

== Problem Instances

// TODO: Check if University of Bologna should be cited
The performance of both the branch-and-bound program and CPLEX model is illustrated here using complex real-world problem instances from a single day of departure operations at London Heathrow as well as benchmark problem instances from Milan Airport. The latter were obtained from the University of Bologna Operations Research Group's freely accessible online #link("https://site.unibo.it/operations-research/en/research/library-of-codes-and-instances-1")[library of instances].

#todo("Write more about problem instances if necessary")

== Comparison of Approaches

#todo("Write about comparison of CPLEX model as well as branch-and-bound, with different de-icing modes")

== Impact

#todo("Write about impact of results")

= Reflections

#todo("Write introduction to reflection")

== Project Management

#todo("Write about project management")

// TODO: Check if a better heading could be used
== Contributions

#todo("Write about LSEPI and contributions")

= References

// NOTE: Title disabled since we want to use a custom title and passing in a heading as the title makes
//       it too big and messes up the table of contents
#bibliography("references.yml", title: none, style: "ieee")