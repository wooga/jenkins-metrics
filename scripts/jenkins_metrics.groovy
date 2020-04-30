import hudson.model.*
import groovy.json.*
def items = Hudson.instance.allItems

def reports = items.collect { item ->
  if (item instanceof Job) {
    def date = new Date()
    def builds = item.getBuilds().limit(200)
    return builds.collect { build ->
      if(!build.isBuilding()) {
        def timings = build.getAction(jenkins.metrics.impl.TimeInQueueAction.class)
        if(timings) {
          report = [:]
          report['build'] = build.toString()
          report['time'] = build.getTime()
          report['duration'] = timings.getTotalDurationMillis()
          report['executing'] = timings.getExecutingTimeMillis()
          report['executorUtilization'] = timings.getExecutorUtilization()

          queuingDetails = [:]
          queuingDetails['duration'] = timings.getQueuingTimeMillis()
          queuingDetails['blocked'] = timings.getBlockedTimeMillis()
          queuingDetails['waiting'] = timings.getWaitingTimeMillis()
          queuingDetails['buildable'] = timings.getBuildableTimeMillis()
          report['queuing'] = queuingDetails
          return report
        }
        return null
      }
      return null
    }
  }
  return null
}.findAll({it != null}).flatten().findAll({it != null}).toList()

println(JsonOutput.toJson(reports))

return
