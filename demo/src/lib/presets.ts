export const presets = [
    {
      name: "Email",
      regex: ".+@.+\\....*",
      cases: [
        { text: "example@example.com", matches: true },
        { text: "user.name+tag+sorting@example.com", matches: true },
        { text: "user.name@example.co.uk", matches: true },
        { text: "user@localserver", matches: false },
        { text: "plainaddress", matches: false },
      ],
    },
    {
      name: "URL",
      regex: "https?:\\/\\/(www\\.)?.+\\...*(\\/.+\\/?)*",
      cases: [
        { text: "https://www.example.com", matches: true },
        { text: "http://example.com", matches: true },
        { text: "www.example.com", matches: false },
        { text: "example.com", matches: false },
        { text: "https://example", matches: false },
      ],
    },
    {
      name: "Contains word 'In'/'in'",
      regex: "(.+ )?(I|i)n( .+)?",
      cases: [
        { text: "He is in the car.", matches: true },
        { text: "Helping people is good.", matches: false },
        { text: "In the future, I want to do stuff.", matches: true },
        { text: "I like pizza.", matches: false },
        { text: "The internet is crazy.", matches: false },
      ],
    },
  ];
