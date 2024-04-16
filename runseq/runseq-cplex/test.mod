main {
  	var src = new IloOplModelSource("runseq.mod");
  	var cplex = new IloCplex();
  	var def = new IloOplModelDefinition(src);
  	
  	var sampleCount = 100;
  	var samples = new Array(sampleCount);
  	for (var n = 1; n <= sampleCount; n += 1) {
  	  	var model = new IloOplModel(def, cplex);
  	  	var data = new IloOplDataSource("runseq.dat");
  	  	model.addDataSource(data);
  	  	model.generate();
  	  	cplex.solve();
  	  	
  	  	var time = cplex.getSolvedTime();
  	  	writeln("runtime for sample " + n + " is " + time);
  	  	samples[n - 1] = time;
  	}
  	samples.sort();
  	
  	var mean = 0;
  	for (var i = 0; i < sampleCount; i += 1) {
  	  	mean += samples[i];
  	}
  	mean /= sampleCount;
  	writeln("mean runtime for " + sampleCount + " samples is " + mean);
  	
  	var stdDev = 0;
  	for (var i = 0; i < sampleCount; i += 1) {
  	  	stdDev += (samples[i] - mean) * (samples[i] - mean);
  	}
  	stdDev /= sampleCount
  	writeln("standard deviation for " + sampleCount + " samples is " + stdDev);
  	
  	function quartile(percent, sampleCount, samples) {
  	  	var index = (sampleCount - 1) * percent;
  	  	var low = samples[Math.floor(index)];
  	  	var high = samples[Math.ceil(index)];
  	  	var fract = index - Math.floor(index);
  	  	return low + (high - low) * fract;
  	}
  	
  	var median = quartile(0.5, sampleCount, samples);
  	writeln("median runtime for " + sampleCount + " samples is " + median);
  	
  	var medAbsDevs = new Array(sampleCount);
  	for (var i = 0; i < sampleCount; i += 1) {
  	  	medAbsDevs[i] = Math.abs(samples[i] - median);
  	}
  	medAbsDevs.sort();
  	
  	var mad = quartile(0.5, sampleCount, medAbsDevs);
  	writeln("median absolute deviation for " + sampleCount + " samples is " + mad);
  	
  	writeln(mean + "," + stdDev + "," + median + "," + mad);
}