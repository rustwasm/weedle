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

[Exposed=System]
partial module m {
  interface A {
  };
  // FIXME: should be m.A
  A getA();
};

[Exposed=System]
partial module m {
  interface B {
  };
  // FIXME: should be m.B
  B getB();
};
