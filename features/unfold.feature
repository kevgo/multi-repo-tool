Feature: unfold folder list

  Rule: replaces the current folder list with all subfolders that match the given condition

    @this
    Scenario:
      Given I am in the "simple" example folder
      And no mrt configuration
      When running "m unfold ls Makefile"
      Then it prints:
        """
        step 1/7: cd {{examples_dir}}/go
        """
      And it returns "success"
