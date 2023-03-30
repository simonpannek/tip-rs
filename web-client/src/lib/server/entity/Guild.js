export default function (sequelize, DataTypes) {
	return sequelize.define(
		'Guild',
		{
			id: { type: DataTypes.BIGINT, primaryKey: true },
			ignore: { type: DataTypes.BOOLEAN, allowNull: false, defaultValue: false },
			default_channel_id: { type: DataTypes.BIGINT, allowNull: true },
			execution_role_id: { type: DataTypes.BIGINT, allowNull: true }
		},
		{
			sequelize,
			tableName: 'guild',
			timestamps: false
		}
	);
}
