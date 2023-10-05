#let todo(message: none) = raw(
    if message == none {
        "// TODO"
    } else {
        "// TODO: " + message
    },
    block: true,
    lang: "rust",
)

#let email(email) = link("mailto:" + email, raw(email))

#set text(font: "EB Garamond", size: 11pt)

#v(1fr)
#align(center)[
    #text(size: 18pt)[*Optimising Aircraft Runway Sequences and De-Icing Schedules*]

    _Project Proposal_

    #v(0.1fr)

    #stack(dir: ltr, spacing: 1fr)[
        Indraneel Mahendrakumar\
        20372495\
        #email("psyim3@nottingham.ac.uk")\
    ][
        _Supervised By_\
        Geert De Maere\
        #email("geert.demaere@nottingham.ac.uk")\
    ]
]
#v(1fr)

// NOTE: Done after cover page since we don't want page numbers to show up on it
#set page(
    numbering: "1",
    number-align: center,
)

#set heading(numbering: "1.1")

#set par(justify: true)

#outline()
#pagebreak()

= Introduction

Airports have a limited number of runways, serving as a bottleneck to the number of aircraft that can take off or land at any given time frame while adhering to strict safety and operational requirements. The efficient scheduling of the take-off and landing of aircraft is thus critical for maximising the capacity of airports, and has a significant impact on operational costs, fuel emissions, and flight delays.

De-icing further complicates this.

Such algorithms must be fast enough and reliable enough to be used in highly dynamic, real-time environments where failure is not an option.

#todo()

= Objectives

#todo()

= Project Plan

#todo()

= References

#todo()