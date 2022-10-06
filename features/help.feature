Feature: display help

  @this
  Scenario: long form
    Given I am in the "simple" example folder
    When running "m help"
    Then it prints:
      """
      Usage: m <command>

      Don't call mrt directly. Run mrt activate | source, then call the shell function m.

      To execute a CLI command in all subfolders: m run <executable> [<arguments>]
      If the given executable fails in one of the subfolders, you end up in that subfolder.
      After investigating/fixing the failure, you can:
      m abort        stops iterating the remaining subfolders
      m retry       retries the failed command
      m ignore      ignores this subfolder and continues in the next subfolder
      m ignore-all  ignores all subsequent failures in all subfolders

      To open a command prompt in all subfolders: m walk
      When you are done with one subfolder, run m next to go to the next subfolder.
      To stop the process early: m abort
      """
