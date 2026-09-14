-- Indonesian bank reference data — the three-digit transfer codes.
--
-- Bank Indonesia assigns each bank a three-digit code (sandi bank) that
-- identifies it in the national transfer system. Employee bank accounts, payroll
-- remittance and supplier payments all key on it, so the list is reference data
-- every tenant needs rather than something each one should retype.
--
-- Scope: the banks a business in Indonesia actually transacts with — the large
-- national banks and their syariah arms, every regional development bank (BPD),
-- the established foreign and joint-venture banks, and the digital banks that
-- now carry meaningful payroll volume.
--
-- Deliberately NOT a transcription of every code ever issued. Published lists
-- still carry banks that no longer exist — Lippo, ABN AMRO, Mutiara, IFI — and
-- seeding a defunct bank is worse than omitting a rare one, because it offers an
-- operator a choice that cannot settle. Names reflect current branding: BII is
-- Maybank, and the three state syariah banks merged into BSI under code 451.
--
-- Deterministic ids derived from the bank code so a second environment resolves
-- the same rows.

INSERT INTO employee.banks (id, code, name, metadata) VALUES
  -- Large national banks
  ('b0000000-0000-4d40-8000-000000000002', '002', 'Bank Rakyat Indonesia (BRI)',        '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000008', '008', 'Bank Mandiri',                        '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000009', '009', 'Bank Negara Indonesia (BNI)',         '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000014', '014', 'Bank Central Asia (BCA)',             '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000200', '200', 'Bank Tabungan Negara (BTN)',          '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000011', '011', 'Bank Danamon',                        '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000013', '013', 'Bank Permata',                        '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000016', '016', 'Bank Maybank Indonesia',              '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000019', '019', 'Bank Panin',                          '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000022', '022', 'Bank CIMB Niaga',                     '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000028', '028', 'Bank OCBC NISP',                      '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000213', '213', 'Bank BTPN',                           '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000426', '426', 'Bank Mega',                           '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000153', '153', 'Bank Sinarmas',                       '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000441', '441', 'Bank Bukopin',                        '{"group":"national"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000003', '003', 'Bank Ekspor Indonesia',               '{"group":"national"}'::jsonb),
  -- Syariah
  ('b0000000-0000-4d40-8000-000000000451', '451', 'Bank Syariah Indonesia (BSI)',        '{"group":"syariah"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000147', '147', 'Bank Muamalat',                       '{"group":"syariah"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000425', '425', 'Bank BCA Syariah',                    '{"group":"syariah"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000506', '506', 'Bank Mega Syariah',                   '{"group":"syariah"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000547', '547', 'Bank BTPN Syariah',                   '{"group":"syariah"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000517', '517', 'Bank Panin Dubai Syariah',            '{"group":"syariah"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000521', '521', 'Bank Syariah Bukopin',                '{"group":"syariah"}'::jsonb),
  -- Regional development banks (BPD)
  ('b0000000-0000-4d40-8000-000000000111', '111', 'Bank DKI',                            '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000112', '112', 'Bank BPD DIY',                        '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000113', '113', 'Bank Jateng',                         '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000114', '114', 'Bank Jatim',                          '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000110', '110', 'Bank BJB (Jabar Banten)',             '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000137', '137', 'Bank Banten',                         '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000116', '116', 'Bank Aceh Syariah',                   '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000117', '117', 'Bank Sumut',                          '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000118', '118', 'Bank Nagari (Sumbar)',                '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000119', '119', 'Bank Riau Kepri',                     '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000120', '120', 'Bank Sumsel Babel',                   '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000121', '121', 'Bank Lampung',                        '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000122', '122', 'Bank Kalsel',                         '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000123', '123', 'Bank Kalbar',                         '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000124', '124', 'Bank Kaltimtara',                     '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000125', '125', 'Bank Kalteng',                        '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000126', '126', 'Bank Sulselbar',                      '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000127', '127', 'Bank SulutGo',                        '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000128', '128', 'Bank NTB Syariah',                    '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000129', '129', 'Bank BPD Bali',                       '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000130', '130', 'Bank NTT',                            '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000131', '131', 'Bank Maluku Malut',                   '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000132', '132', 'Bank Papua',                          '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000133', '133', 'Bank Bengkulu',                       '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000134', '134', 'Bank Sulteng',                        '{"group":"bpd"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000135', '135', 'Bank Sultra',                         '{"group":"bpd"}'::jsonb),
  -- Digital banks
  ('b0000000-0000-4d40-8000-000000000542', '542', 'Bank Jago',                           '{"group":"digital"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000490', '490', 'Bank Neo Commerce',                   '{"group":"digital"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000535', '535', 'SeaBank',                             '{"group":"digital"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000567', '567', 'Allo Bank',                           '{"group":"digital"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000531', '531', 'Bank Amar Indonesia',                 '{"group":"digital"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000523', '523', 'Bank Sahabat Sampoerna',              '{"group":"digital"}'::jsonb),
  -- Foreign and joint-venture
  ('b0000000-0000-4d40-8000-000000000023', '023', 'Bank UOB Indonesia',                  '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000031', '031', 'Citibank',                            '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000041', '041', 'HSBC Indonesia',                      '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000046', '046', 'Bank DBS Indonesia',                  '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000050', '050', 'Standard Chartered Indonesia',        '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000950', '950', 'Bank Commonwealth',                   '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000152', '152', 'Bank Shinhan Indonesia',              '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000164', '164', 'Bank ICBC Indonesia',                 '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000069', '069', 'Bank of China Indonesia',             '{"group":"foreign"}'::jsonb),
  ('b0000000-0000-4d40-8000-000000000212', '212', 'Bank Woori Saudara',                  '{"group":"foreign"}'::jsonb)
ON CONFLICT (id) DO NOTHING;
