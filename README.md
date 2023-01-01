# how-long-now

This project is an attempt to work with timezones. An example use case is to know how much time you have left for an event.

<b>N.B: </b>For now, it uses Utc timezone only.

Usage:

```
cargo run -- -l 12 -m 3 -d 19 -o 4 -y 2023
```

<code>
</div>
<code><b>-l</b></code> is a compulsory argument that must be passed in. It represents the <b>hour</b> of your event.
</div>

<div>
<code><b>-m</b></code> is a compulsory argument that must also be passed in. It represents the <b>minute</b> of your event.
</div>

<div>
<code><b>-o</b></code> is an optional argument. It represents the <b>month</b> of your event with a digit value [0..11].
</div>

<div>
<code><b>-d</b></code> is an optional argument. It represents the <b>day</b> of your event with a digit value [1..31].
</div>

<div>
<code><b>-y</b></code> is an optional argument. It represents the <b>year</b> of your event with any four digit value.
</div>
</code>
