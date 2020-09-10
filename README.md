Jenkins Metrics
===============

A simple CLI tool to calculate basic Jenkins Job KPI's.
The tool needs an exported dataset from Jenkins and displays mean and median values for build duration, queue times etc.
The dataset can be limited by providing a time value and builds can be filtered by providing a prefix pattern.

Installation
------------

_install with brew_

```bash
brew tap wooga/tools
brew install wooga/jenkins-metrics
```

To build from source a recent version of rust is needed. You should use [rustup].

_install from source with cmake_

```bash
git clone git@github.com:wooga/jenkins-metrics.git
cd jenkins-metrics
make install
```

_install from source with cargo_

```bash
git clone git@github.com:wooga/jenkins-metrics.git
cd jenkins-metrics
cargo build --release
#symlink or move binaries in target/release
```

Usage
-----

```
jenkins_metrics - Print basic Jenkins build job metrics.

Usage:
  jenkins_metrics [options] [(--hours=D | --days=D | --weeks=D)] <metrics>
  jenkins_metrics (-h | --help)

Options:
  --filter=F                        simple prefix filter for build names
  --months=D                        data set range in months from today
  --weeks=D                         data set range in weeks from today
  --days=D                          data set range in days from today
  --hours=H                         data set range in hours from today
  --sample-size=S                   Sub sample size in days
  --now                             Use current UTC time as base
  --today                           Include todays date
  -v, --verbose                     print more output
  -d, --debug                       print debug output
  --color WHEN                      Coloring: auto, always, never [default: auto]
  -h, --help                        show this help message and exit
```

The tool requires an exported `metrics` file in the format `json`:

```json
[{
  "build": "name/of/the/jenkins/build/master #74",
  "time": "2019-10-02T11:35:50+0000",
  "duration": 61877,
  "executing": 51805,
  "executorUtilization": 0.84,
  "queuing": {
    "duration": 90,
    "blocked": 0,
    "waiting": 78,
    "buildable": 3
  }
}]
```

This file can be generated with the [`jenkins_metrics.groovy`] script. You can run it with curl:

```
curl --data-urlencode "script=$(< scripts/jenkins_metrics.groovy)" https://jenkins.com/scriptText > jenkins_metrics.json
```

You might need to pass login credentials or other parameters depending on your jenkins setup. Check the [official wiki](https://wiki.jenkins.io/display/JENKINS/Jenkins+Script+Console#JenkinsScriptConsole-Remoteaccess) for more information.

Sub Samples
-----------

By default, only one sample will be generated. The default data range is set to 8 weeks.
If desired its possible to generate a list of sub samples. Use the `--sample-size` flag. This value is measured in days.
So to print the build metrics for each day from the last 8 weeks call the tool like this:

`jenkins_metrics --weeks 8 --sample-size 1 path/to/metrics`

Filtering Builds

The metrics file contains the build name which can be used to filter the reports with a simple prefix.
If you want to print the metrics for all builds starting with `utilities` simply provide this as a `--filter`.

`jenkins_metrics --filter utilities path/to/metrics`

License
=======

Copyright 2017 Wooga GmbH

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

[`jenkins_metrics.groovy`]: scripts/jenkins_metrics.groogy
[rustup]:   https://rustup.rs/
