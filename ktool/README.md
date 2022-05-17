# ktool

Parsing kickertool exported data (.ktool).

There is a demo running `test_2`, output reproduced below. Note the times are unix timestamps in milliseconds.

```
monster dyp
None ["Mario", "Jack"] vs ["Howie", "Brandon"]. Winner: None
None ["Johan", "Steven"] vs ["Eric", "Summer"]. Winner: None
None ["Johan", "Howie"] vs ["Brandon", "Dan"]. Winner: None
None ["Summer", "Todd"] vs ["Steven", "Jack"]. Winner: None
None ["Mario", "Tony"] vs ["Ryan", "Eric"]. Winner: None
Some(1652635505934) ["Eric", "Howie"] vs ["Johan", "Tony"]. Winner: Team1
Some(1652636899038) ["Steven", "Tony"] vs ["Jack", "Johan"]. Winner: Team1
Some(1652636907773) ["Summer", "Howie"] vs ["Eric", "Todd"]. Winner: Team2
Some(1652672823205) ["Howie", "Tony"] vs ["Steven", "Summer"]. Winner: Team1
Some(1652706998463) ["Eric", "Johan"] vs ["Jack", "Todd"]. Winner: Team1
Some(1652707451030) ["Ryan", "Tony"] vs ["Todd", "Dan"]. Winner: Team1
elimination
Some(1652676808047) ["Tony", "Brandon"] vs ["Steven", "Ryan"]. Winner: Team1
Some(1652677037719) ["Howie", "Mario"] vs ["Summer", "Dan"]. Winner: Team2
Some(1652774182035) ["Todd", "Johan"] vs ["Tony", "Brandon"]. Winner: Team2
Some(1652774185135) ["Howie", "Mario"] vs ["Todd", "Johan"]. Winner: Team2
Some(1652774188258) ["Eric", "Jack"] vs ["Summer", "Dan"]. Winner: Team1
Some(1652774190567) ["Steven", "Ryan"] vs ["Summer", "Dan"]. Winner: Team2
Some(1652774192196) ["Tony", "Brandon"] vs ["Eric", "Jack"]. Winner: Team2
Some(1652774199848) ["Summer", "Dan"] vs ["Todd", "Johan"]. Winner: Team1
Some(1652774201703) ["Summer", "Dan"] vs ["Tony", "Brandon"]. Winner: Team2
Some(1652774203987) ["Eric", "Jack"] vs ["Tony", "Brandon"]. Winner: Team2
```
