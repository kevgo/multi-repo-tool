@this
Feature: display help

  Scenario: "help" command
    Given I am in the "simple" example folder
    When running "m help"
    Then it prints:
      """
      Usage: m <command>

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
      To start walking at a specific subfolder: m walk-from <folder name>

      By default, mrt iterates through all direct subfolders of the directory it is called.
      To limit the set of folders that mrt goes through:
      m only <condition>   keeps only the folders for which condition returns exit code 0.
                           You can call "m only" repeatedly to limit by multiple criteria.
      m unfold <condition> replaces the current folder set with all subfolders of the current folders
                           for which the given condition returns exit code 0.
      """

  Scenario: no command
    Given I am in the "simple" example folder
    When running "m"
    Then it prints:
      """
      ERROR: no command provided

      Usage: m <command>

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
      To start walking at a specific subfolder: m walk-from <folder name>

      By default, mrt iterates through all direct subfolders of the directory it is called.
      To limit the set of folders that mrt goes through:
      m only <condition>   keeps only the folders for which condition returns exit code 0.
                           You can call "m only" repeatedly to limit by multiple criteria.
      m unfold <condition> replaces the current folder set with all subfolders of the current folders
                           for which the given condition returns exit code 0.
      """

  Scenario: wrong command
    Given I am in the "simple" example folder
    When running "m zonk"
    Then it prints:
      """
      ERROR: unknown command: zonk

      Usage: m <command>

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
      To start walking at a specific subfolder: m walk-from <folder name>

      By default, mrt iterates through all direct subfolders of the directory it is called.
      To limit the set of folders that mrt goes through:
      m only <condition>   keeps only the folders for which condition returns exit code 0.
                           You can call "m only" repeatedly to limit by multiple criteria.
      m unfold <condition> replaces the current folder set with all subfolders of the current folders
                           for which the given condition returns exit code 0.
      """
