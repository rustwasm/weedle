[Exposed=Window, SecureContext]
module temporal {
  interface Timezone {
    readonly attribute USVString name;
    long long offsetMs(long long unixTime);
  };
  temporal.Timezone getCurrentTimezone();
  readonly attribute temporal.Timezone initialTimezone;
};

[Exposed=System]
partial module m {
  interface A {
  };
  m.A getA();
};

[Exposed=System]
partial module m {
  interface B {
  };
  m.B getB();
};
