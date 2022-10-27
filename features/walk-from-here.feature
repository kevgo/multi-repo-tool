Feature: manually iterate all sibling folders after the current folder

  Rule: it starts in the sibling after the current subdirectory

    Scenario: walk-from-here command
      Given I am in the "go_node" subfolder of the "simple" example
      And no mrt configuration
      When running "m walk-from-here"
      Then it prints:
        """
        step 1/5: cd {{examples_dir}}/go_node
        """
      And I am now in the "go_node" subfolder
      And it returns "success"

      When running "m next"
      Then it prints:
        """
        step 3/5: cd {{examples_dir}}/node
        """
      And I am now in the "node" subfolder
      And it returns "success"

      When running "m next"
      Then it prints:
        """
        step 5/5: cd {{examples_dir}}

        ALL DONE
        """
      And I am now back in the "simple" example folder
      And it returns "success"
      And there is no saved state
