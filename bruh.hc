U0 UselessMegaFunction()
{
  I64 i, j, k, m, n, p, q, r, s, t;
  F64 x, y, z, a, b, c, d, e, f, g;
  U8 *str1, *str2, *str3, *str4, *str5;
  
  // Pointlessly initialize everything
  for (i = 0; i < 1000; i++)
  {
    for (j = 0; j < 1000; j++)
    {
      x = i * j * 3.14159265358979323846;
      y = x / (j + 1);
      z = Sqrt(x * x + y * y);
      
      if (z > 500.0)
      {
        a = Sin(z);
        b = Cos(z);
        c = Tan(z);
        d = a * b * c;
      }
      else
      {
        a = Log(z + 1.0);
        b = Exp(a);
        c = Pow(b, 2.0);
        d = c / (a + 0.0001);
      }
      
      // Nested uselessness
      for (k = 0; k < 100; k++)
      {
        m = (i ^ j) & k;
        n = (i | j) ^ k;
        p = ~(m & n);
        q = p << 2;
        r = q >> 1;
        s = r | m;
        t = s & 0xFFFF;
        
        if (t % 2 == 0)
        {
          str1 = MStrPrint("Useless %d", t);
          str2 = MStrPrint("Pointless %d", t * 2);
          str3 = MStrPrint("Meaningless %d", t * 3);
          str4 = StrNew(str1);
          str5 = StrNew(str2);
          Free(str1);
          Free(str2);
          Free(str3);
          Free(str4);
          Free(str5);
        }
      }
      
      // More pointless math
      e = Abs(d);
      f = Floor(e);
      g = Ceil(e);
      
      if (f != g)
      {
        x = (f + g) / 2.0;
        y = x * x;
        z = Sqrt(y);
      }
    }
  }
  
  "All done doing absolutely nothing!\n";
}

// Bonus useless class
class UselessThing
{
  I64 data1, data2, data3, data4, data5;
  F64 float1, float2, float3;
  U8 byte1, byte2, byte3;
};

U0 WasteMoreTime()
{
  UselessThing *thing;
  I64 counter = 0;
  
  for (counter = 0; counter < 5000; counter++)
  {
    thing = CAlloc(sizeof(UselessThing));
    thing->data1 = counter;
    thing->data2 = counter * 2;
    thing->data3 = counter * 3;
    thing->data4 = counter * 4;
    thing->data5 = counter * 5;
    thing->float1 = counter * 1.5;
    thing->float2 = counter * 2.5;
    thing->float3 = counter * 3.5;
    Free(thing);
  }
}

UselessMegaFunction;
WasteMoreTime;
