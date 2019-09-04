[Exposed=Window, SecureContext]
module temporal {
  interface Timezone {
    readonly attribute USVString name;
    long long offsetMs(long long unixTime);
  };
  // FIXME: should be temporal.Timezone but we don't parse scoped ids yet
  Timezone getCurrentTimezone();
  // FIXME: should be temporal.Timezone but we don't parse scoped ids yet
  readonly attribute Timezone initialTimezone;
};
