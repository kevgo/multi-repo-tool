Feature: manually iterate all folders

  Rule: it iterates all subdirectories

    Scenario: walk command
      Given I am in the "simple" example folder
      And no mrt configuration
      When running "m walk"
      Then it prints:
        """
        step 1/7: cd {{examples_dir}}/go
        """
      And I am now in the "go" subfolder
      And it returns "success"

      When running "m next"
      Then it prints:
        """
        step 3/7: cd {{examples_dir}}/go_node
        """
      And I am now in the "go_node" subfolder
      And it returns "success"

      When running "m next"
      Then it prints:
        """
        step 5/7: cd {{examples_dir}}/node
        """
      And I am now in the "node" subfolder
      And it returns "success"

      When running "m next"
      Then it prints:
        """
        step 7/7: cd {{examples_dir}}

        ALL DONE
        """
      And I am now back in the "simple" example folder
      And it returns "success"
      And there is no saved state
