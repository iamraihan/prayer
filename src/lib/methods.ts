/**
 * Aladhan calculation methods.
 * https://aladhan.com/prayer-times-api — `method` query parameter.
 */
export interface CalcMethod {
  id: number;
  name: string;
}

export const METHODS: CalcMethod[] = [
  { id: 3, name: "Muslim World League" },
  { id: 1, name: "University of Islamic Sciences, Karachi" },
  { id: 4, name: "Umm Al-Qura University, Makkah" },
  { id: 5, name: "Egyptian General Authority of Survey" },
  { id: 2, name: "Islamic Society of North America (ISNA)" },
  { id: 15, name: "Moonsighting Committee Worldwide" },
  { id: 16, name: "Dubai (UAE)" },
  { id: 8, name: "Gulf Region" },
  { id: 9, name: "Kuwait" },
  { id: 10, name: "Qatar" },
  { id: 17, name: "JAKIM, Malaysia" },
  { id: 20, name: "KEMENAG, Indonesia" },
  { id: 11, name: "Majlis Ugama Islam Singapura, Singapore" },
  { id: 13, name: "Diyanet İşleri Başkanlığı, Turkey" },
  { id: 12, name: "Union Organization Islamic de France" },
  { id: 14, name: "Spiritual Administration of Muslims of Russia" },
  { id: 23, name: "Jordan" },
  { id: 21, name: "Morocco" },
  { id: 19, name: "Algeria" },
  { id: 18, name: "Tunisia" },
  { id: 7, name: "Institute of Geophysics, University of Tehran" },
  { id: 22, name: "Comunidade Islamica de Lisboa" },
  { id: 0, name: "Shia Ithna-Ansari" },
];

/**
 * Sensible default method per country (ISO code).
 * The wizard pre-selects this; the user can always override.
 */
export const METHOD_BY_COUNTRY: Record<string, number> = {
  AE: 16, AF: 1, AL: 3, AR: 3, AT: 3, AU: 3, AZ: 3,
  BD: 1, BE: 3, BH: 8, BI: 1, BN: 1, BW: 3,
  CA: 2, CH: 3, CN: 3, CY: 13, CZ: 3, DE: 3, DK: 3,
  DZ: 19, EG: 5, ES: 3, ET: 3,
  FI: 3, FR: 12,
  GB: 3, GE: 3, GH: 3, GR: 3,
  HR: 3, HU: 3,
  ID: 20, IE: 3, IN: 1, IQ: 1, IR: 7, IT: 3,
  JO: 23,
  KE: 3, KG: 3, KW: 9, KZ: 3,
  LB: 3, LY: 3, LT: 3, LU: 3, LV: 3,
  MA: 21, MK: 3, MM: 1, MT: 3, MV: 15, MY: 17,
  NG: 3, NL: 3, NO: 3, NZ: 3,
  OM: 4,
  PK: 1, PH: 3, PL: 3, PS: 3, PT: 3,
  QA: 10,
  RO: 3, RU: 14,
  SA: 4, SD: 3, SE: 3, SG: 11, SI: 3, SK: 3, SO: 3, SY: 3,
  TH: 15, TN: 18, TR: 13, TZ: 3,
  UA: 3, US: 2, UZ: 3,
  YE: 3, ZA: 3,
};

export function methodById(id: number): CalcMethod | undefined {
  return METHODS.find((m) => m.id === id);
}

export function methodName(id: number): string {
  return methodById(id)?.name ?? `Method ${id}`;
}
