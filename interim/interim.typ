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

= References

#bibliography("references.yml", title: none)