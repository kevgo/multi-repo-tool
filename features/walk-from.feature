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

        Usage: m walk-from <folder to start the walk in>
        """
      And it returns "failure"
