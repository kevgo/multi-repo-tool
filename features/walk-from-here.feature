Feature: manually iterate all sibling folders after the current folder

  Rule: it starts in the sibling after the current subdirectory

    @this
    Scenario: walk-from-here command
      Given I am in the "go" subfolder of the "simple" example
      And no mrt configuration
      When running "m walk-from-here"
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
