-- Add date_of_exit to employments so the offboarding handler can set it
-- (OffboardingClosedHandler updates status='inactive' + date_of_exit=last_working_day).
ALTER TABLE employee.employments ADD COLUMN IF NOT EXISTS date_of_exit date;
