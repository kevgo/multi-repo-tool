Feature: manually iterate all folders

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: without argument it iterates all subdirectories

    Scenario: no argument
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

  Rule: with argument it starts at the given subdirectory

    @this
    Scenario: argument
      When running "m walk node"
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
