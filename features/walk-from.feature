Feature: manually iterate all folders starting at a given folder

  Rule: it starts at the given subdirectory

    Scenario: folder name given
      Given I am in the "simple" example folder
      And no mrt configuration
      When running "m walk-from node"
      Then it prints:
        """
        step 1/3: cd {{examples_dir}}/node
        """
      And I am now in the "node" subfolder
      And it returns "success"

      When running "m next"
      Then it prints:
        """
        step 3/3: cd {{examples_dir}}

        ALL DONE
        """
      And I am now back in the "simple" example folder
      And it returns "success"
      And there is no saved state

    Scenario: folder name missing
      Given I am in the "simple" example folder
      And no mrt configuration
      When running "m walk-from"
      Then it prints:
        """
        ERROR: missing start folder

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
        """
      And it returns "failure"
