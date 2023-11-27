#let email(email) = link("mailto:" + email, raw(email))

#set text(font: "EB Garamond", size: 11pt)

#v(1fr)
#align(center)[
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    #text(size: 13pt)[_COMP3003 Interim Report_]

    #v(0.2fr)

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

// NOTE: Done after cover page since we don't want page numbers to show up on it
#set page(numbering: "1")

#set heading(numbering: "1.1")
#show heading: set block(above: 2em, below: 1.3em)

#set par(justify: true)

#outline(indent: auto)
#pagebreak()

// TODO: Add introduction from proposal and tweak it
= Introduction

== Background

== Motivation

= Objectives

The primary aim of this project is to investigate the integrated runway sequencing and de-icing problem by developing three algorithms that explore four different approaches to the order of aircraft de-icing. The investigation and implementation of these algorithms will provide deeper insights into the problem's fundamental characteristics and the interactions between runway sequencing and de-icing, as well as the potential benefits of integrating their solutions.

The project's key objectives are as follows:

1. *Investigate prior approaches to runway sequencing*. The mathematical models and formulations proposed in prior research may not be directly applicable to this project, as they focus on only runway sequencing or only de-icing. Thus, there will be a need to understand and then adapt or extend these models so they are suitable for the integrated problem.

2. *Design and implement three algorithms* -- branch-and-bound, branch-and-bound with a rolling window, and mathematical programming -- *using four different de-icing ordering approaches* -- sequential, based on the Target Off-Block Time (TOBT), based on the Calculated Take-Off Time (CTOT), and based on runway sequences. The algorithms must be generic enough to work with data from different sources (i.e., different airports and datasets), by using a set of common features and characteristics in the data. Additionally, they must be fast and reliable enough to be viable in highly dynamic, real-time situations where unexpected failure is not an option @demaere-pruning-rules. If time permits, a fourth algorithm -- dynamic programming -- may also be explored, since it is known to work well for runway sequencing @lieder-dynamic-programming but its effectiveness at de-icing is yet to be evaluated.

3. *Analyse the algorithms' performance and outputs*. This will involve benchmarking them on known and available datasets and comparing them with existing solutions as well as with each other. A simulation that is more representative of real-world data and use cases will also be used to run the algorithms on multiple problem instances over a longer period of time. This will help expose any issues, such as instability in the generated sequences, that may not be visible in individual runs.

4. *Develop a tool for visualising the outputs and intermediate results produced by the algorithms*. This will provide a more intuitive, human-friendly view intended to aid users' understanding, which will not only be useful for an end user, but also for the analysis of the algorithms themselves.

= Implementation

// TODO: Review and check if the Rust website should be cited
For this project, I have opted to use #link("https://www.rust-lang.org")[Rust]. The primary reason for this is my familiarity and experience with the language, which allows me to be more confident in my implementation and estimated timelines. Another major factor is that Rust's rich type system and unique memory ownership and borrowing mechanics eliminate many classes of bugs -- such as null reference exceptions or Undefined Behaviour -- at compile time. As a result, I can be more productive while being confident in my implementation's reliability and handling of edge cases.

== Data

The datasets as well as the ICAO separations between each pair of aircraft used for this project were obtained from the University of Bologna Operations Research Group's freely accessible online #link("https://site.unibo.it/operations-research/en/research/library-of-codes-and-instances-1")[library of codes and instances]. These instance sets were also used in previous works @furini-improved-horizon. // TODO: Reference Furini's earlier paper

=== Data Generation

The aforementioned datasets were developed solely for the runway sequencing problem and not integrated runway and de-icing sequencing. This meant that the instances did not contain data for the pushback durations, taxi durations, de-icing durations, and line-up durations of aircraft, making them largely unsuitable for use as-is in this project. Thus, there was a need to augment the datasets and create a data generator.

// TODO: Talk about creating the new CSV format and generating random data

=== Data Preprocessing

// TODO: Probably reference the paper by Psaraftis
Before sequencing, an instance is split into sets of _separation-identical_ aircraft as a preprocessing step. Two aircraft $A_i$ and $A_j$ are separation-identical if their mutual separations with respect to every other aircraft $A_k$ in the set of aircraft $A$ are the same @demaere-pruning-rules; i.e. $A_i$ and $A_j$ are separation-identical if and only if:

// TODO: Figure out how to number equations
$
forall_(A_k in A), A_k != A_i and A_k != A_j and delta_(i k) = delta_(j k) and delta_(k i) = delta_(k j)
$

This allows the exploitation of _complete orders_ between separation-identical aircraft. A complete order exists between two aircraft $A_i$ and $A_j$ if any arbitrary sequence containing $A_i$ and $A_j$ cannot be improved any further by reversing the orders of $A_i$ and $A_j$ @demaere-pruning-rules. Such complete orders simplify the problem of runway sequencing to one of interleaving ordered sets of separation-identical aircraft. It also reduces the problem's worst-case computational complexity from $n!$ to $O(m^2(n + 1)^m)$, where $n$ denotes the number of sets and $m$ denotes the number of aircraft @demaere-pruning-rules.

Separation-identical sets are generated by comparing the separations of every pair of aircraft with every other aircraft in $A$ as follows:

#import "@preview/lovelace:0.1.0": *

#let pseudocode = pseudocode.with(indentation-guide-stroke: 0.1pt)

// TODO: Use `algorithm` when double heading issue is fixed - see https://github.com/andreasKroepelin/lovelace/pull/1
// TODO: Maybe add some comments
#pseudocode(
    [*input*: set of aircraft $A$],
    [*output*: separation-identical sets of aircraft in $A$],

    [$S <-$ empty list],
    [*for* $A_i$ *in* $A$ *do*], ind,
        [*for* $S_n$ *in* $S$ *do*], ind,
            [*for* $A_j$ *in* $S_n$ *do*], ind,
                [*for* $A_k$ *in* $A$ except $A_i, A_j$ *do*], ind,

                    [*if* $delta_(i k) != delta_(j k)$ *or* $delta_(k i) != delta_(k j)$ *then*], ind,
                        [*continue* to next set in $S$], ded,
                    [*end*], ded,

                [*end*], ded,
            [*end*],
            
            [add $A_i$ to $S_n$],
            [*continue* to next aircraft in $A$], ded,

        [*end*],

        [$s <-$ singleton with $A_i$],
        [add $s$ to $S$], ded,

    [*end*], ded,

    [*return* $S$],
)

// TODO: Review if this paragraph is needed and probably cite Psaraftis
Since all of the methods used in this project are exact methods, using separation-identical sets does not compromise the optimality of the generated sequences @demaere-pruning-rules, and considerably trims the solution search space. At the same time, the efficiency of exploiting complete orders is highly dependent on the separations between aircraft and the diversity of aircraft. In practice, complete orders can be exploited well due to the typical separation matrices and aircraft diversity in runway sequencing instances. However, in some cases -- such as when every aircraft is subject to a CTOT or when there are very few separation-identical aircraft -- the number of separation sets might be too large and the number of aircraft in each set too small. Such cases prevent the effective exploitation of complete orders @demaere-pruning-rules.

// TODO: Talk about the branch-and-bound implementation
== Branch-and-bound

The current lower bound is updated in each iteration and passed around as a parameter to avoid having to re-calculate it from scratch every iteration. This leads to a noticeable decrease in run time, especially for larger instances with more aircraft to sequence.

// TODO: Rework to make less messy and more consistent
#pseudocode(
    [*if* length of $D$ $=$ length of $A$ *then*], ind,
        [$L <-$ minimum of $L$ and $l$],
        [$b <- D$],
        [*return*], ded,
    [*end*],
    [*for* $(S_n, I_n)$ *in* $S$ zipped with $I$ *do*], ind,
        [*if* $I_n >=$ length of $S_n$ *then*], ind,
            [*continue*], ded,
        [*end*],

        [$a <-$ aircraft at index $I_n$ in $S_n$],
        [$d <-$ schedule departure for $a$],

        [$c <-$ cost of $d$],
        [*if* $l + c > L$ *then*], ind,
            [*continue*], ded,
        [*end*],

        [add $d$ to $D$],
        [$l <- l + c$],
        [$I_n <- I_n + 1$],

        [*recurse* with updated parameters],

        [remove $d$ from $D$],
        [$l <- l - c$],
        [$I_n <- I_n - 1$], ded,
    [*end*],
)

// TODO: Talk about the visualiser implementation
== Visualising Sequences

// TODO: Talk about progress and reflections including LSEPI
= Progress

= References

#bibliography("references.yml", title: none)